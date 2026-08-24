use std::ffi::c_char;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct PlayerFlags {
    pub player_ptr: *const std::ffi::c_void,
    pub flags: Vec<u8>,
    pub count: usize,
}

unsafe impl Send for PlayerFlags {}
unsafe impl Sync for PlayerFlags {}

impl PlayerFlags {
    pub fn new(player_ptr: *const std::ffi::c_void, count: usize) -> Self {
        let aligned_count = (count + 8 - 1) & !(8 - 1);
        Self {
            player_ptr,
            flags: vec![0u8; aligned_count / 8],
            count: aligned_count,
        }
    }
}

pub struct PlayerFlagsManager {
    pub flag_index_size: usize,
    pub player_flags: Vec<PlayerFlags>,
}

impl PlayerFlagsManager {
    pub fn new(flag_index_size: usize) -> Self {
        Self {
            flag_index_size,
            player_flags: Vec::new(),
        }
    }

    pub fn add_player<F>(&mut self, player_ptr: *const std::ffi::c_void, is_same_system: F)
    where
        F: Fn(*const std::ffi::c_void, *const std::ffi::c_void) -> bool,
    {
        let mut entry = PlayerFlags::new(player_ptr, self.flag_index_size);
        for existing in &self.player_flags {
            if is_same_system(player_ptr, existing.player_ptr) {
                let copy_len = entry.flags.len().min(existing.flags.len());
                entry.flags[..copy_len].copy_from_slice(&existing.flags[..copy_len]);
                break;
            }
        }
        self.player_flags.push(entry);
    }

    pub fn reset(&mut self) {
        self.player_flags.clear();
    }

    pub fn set_flag<F>(&mut self, player_ptr: *const std::ffi::c_void, index: usize, is_same_system: F)
    where
        F: Fn(*const std::ffi::c_void, *const std::ffi::c_void) -> bool,
    {
        if index >= self.flag_index_size || player_ptr.is_null() {
            return;
        }
        let byte_idx = index / 8;
        let bit_mask = 128 >> (index % 8);

        for entry in &mut self.player_flags {
            if is_same_system(player_ptr, entry.player_ptr) {
                if byte_idx < entry.flags.len() {
                    entry.flags[byte_idx] |= bit_mask;
                }
            }
        }
    }

    pub fn get_flag(&self, player_ptr: *const std::ffi::c_void, index: usize) -> bool {
        if index >= self.flag_index_size || player_ptr.is_null() {
            return false;
        }
        let byte_idx = index / 8;
        let bit_mask = 128 >> (index % 8);

        for entry in &self.player_flags {
            if entry.player_ptr == player_ptr {
                if byte_idx < entry.flags.len() {
                    return (entry.flags[byte_idx] & bit_mask) != 0;
                }
            }
        }
        false
    }
}

pub fn gather_rtt_stats<FLocal, FRtt>(player_count: u32, is_local: FLocal, get_rtt: FRtt) -> String
where
    FLocal: Fn(u32) -> bool,
    FRtt: Fn(u32) -> i32,
{
    let mut stats = String::from("Rtt: ");
    for i in 0..player_count {
        if !is_local(i) {
            let rtt = get_rtt(i);
            stats.push_str(&format!("{}: {}/", i, rtt));
        }
    }
    stats
}

struct GlobalNetworkState {
    flags_mgr: PlayerFlagsManager,
    game_running: bool,
    leaving_game: bool,
    is_offline_game: bool,
    is_private_game: bool,
    host_changed: bool,
}

static NETWORK_STATE: Mutex<GlobalNetworkState> = Mutex::new(GlobalNetworkState {
    flags_mgr: PlayerFlagsManager {
        flag_index_size: 0,
        player_flags: Vec::new(),
    },
    game_running: false,
    leaving_game: false,
    is_offline_game: false,
    is_private_game: false,
    host_changed: false,
});

type SameSystemFn = extern "C" fn(p1: *const std::ffi::c_void, p2: *const std::ffi::c_void) -> bool;

#[no_mangle]
pub extern "C" fn rust_stub_network_system_flag_init(flag_index_size: usize) {
    let mut state = NETWORK_STATE.lock().unwrap();
    state.flags_mgr = PlayerFlagsManager::new(flag_index_size);
}

#[no_mangle]
pub extern "C" fn rust_stub_network_system_flag_add_player(
    player_ptr: *const std::ffi::c_void,
    same_system_fn: SameSystemFn,
) {
    let mut state = NETWORK_STATE.lock().unwrap();
    state.flags_mgr.add_player(player_ptr, |p1, p2| same_system_fn(p1, p2));
}

#[no_mangle]
pub extern "C" fn rust_stub_network_system_flag_reset() {
    let mut state = NETWORK_STATE.lock().unwrap();
    state.flags_mgr.reset();
}

#[no_mangle]
pub extern "C" fn rust_stub_network_system_flag_set(
    player_ptr: *const std::ffi::c_void,
    index: i32,
    same_system_fn: SameSystemFn,
) {
    if index < 0 {
        return;
    }
    let mut state = NETWORK_STATE.lock().unwrap();
    state.flags_mgr.set_flag(player_ptr, index as usize, |p1, p2| same_system_fn(p1, p2));
}

#[no_mangle]
pub extern "C" fn rust_stub_network_system_flag_get(
    player_ptr: *const std::ffi::c_void,
    index: i32,
) -> bool {
    if index < 0 {
        return false;
    }
    let state = NETWORK_STATE.lock().unwrap();
    state.flags_mgr.get_flag(player_ptr, index as usize)
}

#[no_mangle]
pub extern "C" fn rust_stub_network_gather_rtt_stats(
    player_count: u32,
    is_local_fn: extern "C" fn(u32) -> bool,
    get_rtt_fn: extern "C" fn(u32) -> i32,
    out_buf: *mut c_char,
    out_capacity: usize,
) {
    if out_buf.is_null() || out_capacity == 0 {
        return;
    }

    let result = gather_rtt_stats(player_count, |i| is_local_fn(i), |i| get_rtt_fn(i));
    let bytes = result.as_bytes();
    let copy_len = bytes.len().min(out_capacity.saturating_sub(1));
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_buf as *mut u8, copy_len);
        *out_buf.add(copy_len) = 0;
    }
}

#[no_mangle]
pub extern "C" fn rust_stub_network_set_local_game(is_local: bool) -> bool {
    let mut state = NETWORK_STATE.lock().unwrap();
    state.is_offline_game = is_local;
    true
}

#[no_mangle]
pub extern "C" fn rust_stub_network_set_private_game(is_private: bool) {
    let mut state = NETWORK_STATE.lock().unwrap();
    state.is_private_game = is_private;
}

#[no_mangle]
pub extern "C" fn rust_stub_network_is_host() -> bool {
    let state = NETWORK_STATE.lock().unwrap();
    !state.host_changed
}

#[no_mangle]
pub extern "C" fn rust_stub_network_is_in_session() -> bool {
    let state = NETWORK_STATE.lock().unwrap();
    state.game_running
}

#[no_mangle]
pub extern "C" fn rust_stub_network_set_game_running(running: bool) {
    let mut state = NETWORK_STATE.lock().unwrap();
    state.game_running = running;
}

#[no_mangle]
pub extern "C" fn rust_stub_network_is_leaving_game() -> bool {
    let state = NETWORK_STATE.lock().unwrap();
    state.leaving_game
}

#[no_mangle]
pub extern "C" fn rust_stub_network_set_leaving_game(leaving: bool) {
    let mut state = NETWORK_STATE.lock().unwrap();
    state.leaving_game = leaving;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_flags_alignment_and_bit_ops() {
        let dummy_p1 = 0x1000 as *const std::ffi::c_void;
        let dummy_p2 = 0x2000 as *const std::ffi::c_void;

        let mut mgr = PlayerFlagsManager::new(10); // requested 10 flags -> aligned to 16 bits (2 bytes)
        mgr.add_player(dummy_p1, |p1, p2| p1 == p2);

        assert!(!mgr.get_flag(dummy_p1, 0));
        assert!(!mgr.get_flag(dummy_p1, 5));

        mgr.set_flag(dummy_p1, 5, |p1, p2| p1 == p2);
        assert!(mgr.get_flag(dummy_p1, 5));
        assert!(!mgr.get_flag(dummy_p1, 0));

        // Add dummy_p2 on same system as dummy_p1
        mgr.add_player(dummy_p2, |_, _| true);
        assert!(mgr.get_flag(dummy_p2, 5)); // copied existing flags

        mgr.reset();
        assert!(!mgr.get_flag(dummy_p1, 5));
    }

    #[test]
    fn test_gather_rtt_stats() {
        let stats = gather_rtt_stats(
            3,
            |idx| idx == 0,         // player 0 is local
            |idx| (idx * 50) as i32 // player 1 rtt 50, player 2 rtt 100
        );

        assert_eq!(stats, "Rtt: 1: 50/2: 100/");
    }
}
