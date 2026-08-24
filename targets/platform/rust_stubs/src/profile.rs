use std::ffi::c_char;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;

pub const FAKE_XUID_BASE: u64 = 0xe000d45248242f2e;
pub const XUSER_MAX_COUNT: usize = 4;
pub const MAX_FAVORITE_SKINS: usize = 10;
pub const TUTORIAL_PROFILE_STORAGE_BYTES: usize = 64;

pub const GAMESETTING_CLOUDS: u32 = 0x00000001;
pub const GAMESETTING_ONLINE: u32 = 0x00000002;
pub const GAMESETTING_FRIENDSOFFRIENDS: u32 = 0x00000008;
pub const GAMESETTING_DISPLAYUPDATEMSG: u32 = 0x00000030;
pub const GAMESETTING_BEDROCKFOG: u32 = 0x00000040;
pub const GAMESETTING_DISPLAYHUD: u32 = 0x00000080;
pub const GAMESETTING_DISPLAYHAND: u32 = 0x00000100;
pub const GAMESETTING_CUSTOMSKINANIM: u32 = 0x00000200;
pub const GAMESETTING_DEATHMESSAGES: u32 = 0x00000400;
pub const GAMESETTING_UISIZE: u32 = 0x00001800;
pub const GAMESETTING_UISIZE_SPLITSCREEN: u32 = 0x00006000;
pub const GAMESETTING_ANIMATEDCHARACTER: u32 = 0x00008000;
pub const GAMESETTING_PS3EULAREAD: u32 = 0x00010000;
pub const GAMESETTING_PSVITANETWORKMODEADHOC: u32 = 0x00020000;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct ProfileSettings {
    pub i_y_axis_inversion: i32,
    pub i_controller_sensitivity: i32,
    pub i_vibration: i32,
    pub b_swap_sticks: bool,
}

#[repr(C)]
#[derive(Debug)]
pub struct ProfileGameSettings {
    pub b_settings_changed: u8,
    pub uc_music_volume: u8,
    pub uc_sound_fx_volume: u8,
    pub uc_sensitivity: u8,
    pub uc_gamma: u8,
    pub uc_pad01: u8,
    pub us_bitmask_values: u16,
    pub ui_debug_bitmask: u32,
    pub uc_tutorial_completion: [u8; TUTORIAL_PROFILE_STORAGE_BYTES],
    pub dw_selected_skin: u32,
    pub uc_menu_sensitivity: u8,
    pub uc_interface_opacity: u8,
    pub uc_pad02: u8,
    pub us_pad03: u8,
    pub ui_bitmask_values: u32,
    pub ui_special_tutorial_bitmask: u32,
    pub dw_selected_cape: u32,
    pub ui_favorite_skin_a: [u32; MAX_FAVORITE_SKINS],
    pub uc_current_favorite_skin_pos: u8,
    pub _pad_align: [u8; 3],
    pub ui_mash_up_pack_worlds_display: u32,
    pub uc_language: u8,
    pub _pad_reserved: [u8; 59],
}

impl Default for ProfileGameSettings {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

const _: () = {
    assert!(std::mem::size_of::<ProfileGameSettings>() == 204);
};

pub fn initialise_default_game_settings(game_settings: &mut ProfileGameSettings) {
    game_settings.uc_menu_sensitivity = 100;
    game_settings.uc_interface_opacity = 80;
    game_settings.us_bitmask_values |= 0x0200;
    game_settings.us_bitmask_values |= 0x0400;
    game_settings.us_bitmask_values |= 0x1000;
    game_settings.us_bitmask_values |= 0x8000;
    game_settings.ui_bitmask_values = 0;
    game_settings.ui_bitmask_values |= GAMESETTING_CLOUDS;
    game_settings.ui_bitmask_values |= GAMESETTING_ONLINE;
    game_settings.ui_bitmask_values |= GAMESETTING_FRIENDSOFFRIENDS;
    game_settings.ui_bitmask_values |= GAMESETTING_DISPLAYUPDATEMSG;
    game_settings.ui_bitmask_values &= !GAMESETTING_BEDROCKFOG;
    game_settings.ui_bitmask_values |= GAMESETTING_DISPLAYHUD;
    game_settings.ui_bitmask_values |= GAMESETTING_DISPLAYHAND;
    game_settings.ui_bitmask_values |= GAMESETTING_CUSTOMSKINANIM;
    game_settings.ui_bitmask_values |= GAMESETTING_DEATHMESSAGES;
    game_settings.ui_bitmask_values |= GAMESETTING_UISIZE & 0x00000800;
    game_settings.ui_bitmask_values |= GAMESETTING_UISIZE_SPLITSCREEN & 0x00004000;
    game_settings.ui_bitmask_values |= GAMESETTING_ANIMATEDCHARACTER;

    for i in 0..MAX_FAVORITE_SKINS {
        game_settings.ui_favorite_skin_a[i] = 0xFFFFFFFF;
    }

    game_settings.uc_current_favorite_skin_pos = 0;
    game_settings.ui_mash_up_pack_worlds_display = 0xFFFFFFFF;
    game_settings.ui_bitmask_values &= !GAMESETTING_PS3EULAREAD;
    game_settings.uc_language = 0;
    game_settings.ui_bitmask_values &= !GAMESETTING_PSVITANETWORKMODEADHOC;
    game_settings.uc_tutorial_completion[0] = 0xFF;
    game_settings.uc_tutorial_completion[1] = 0xFF;
    game_settings.uc_tutorial_completion[2] = 0x0F;
    game_settings.uc_tutorial_completion[28] |= 1 << 0;
}

extern "C" {
    fn malloc(size: usize) -> *mut std::ffi::c_void;
    fn memset(ptr: *mut std::ffi::c_void, val: i32, size: usize) -> *mut std::ffi::c_void;
}

static LOCKED_PROFILE: AtomicI32 = AtomicI32::new(0);

struct StaticProfileState {
    dashboard_settings: [ProfileSettings; XUSER_MAX_COUNT],
    gamertags: [[c_char; 16]; XUSER_MAX_COUNT],
    display_names: [String; XUSER_MAX_COUNT],
    profile_data_ptrs: [*mut u8; XUSER_MAX_COUNT],
    profile_data_sizes: [usize; XUSER_MAX_COUNT],
}

unsafe impl Send for StaticProfileState {}
unsafe impl Sync for StaticProfileState {}

static PROFILE_STATE: Mutex<StaticProfileState> = Mutex::new(StaticProfileState {
    dashboard_settings: [ProfileSettings {
        i_y_axis_inversion: 0,
        i_controller_sensitivity: 0,
        i_vibration: 0,
        b_swap_sticks: false,
    }; XUSER_MAX_COUNT],
    gamertags: [[0; 16]; XUSER_MAX_COUNT],
    display_names: [String::new(), String::new(), String::new(), String::new()],
    profile_data_ptrs: [std::ptr::null_mut(); XUSER_MAX_COUNT],
    profile_data_sizes: [0; XUSER_MAX_COUNT],
});

fn ensure_fake_identity_impl(state: &mut StaticProfileState, pad: usize) {
    if pad >= XUSER_MAX_COUNT || state.gamertags[pad][0] != 0 {
        return;
    }
    let tag = format!("Player{}\0", pad + 1);
    let bytes = tag.as_bytes();
    for (i, &b) in bytes.iter().enumerate().take(16) {
        state.gamertags[pad][i] = b as c_char;
    }
    state.display_names[pad] = format!("Player{}", pad + 1);
}

#[no_mangle]
pub extern "C" fn rust_stub_profile_initialise(game_defined_data_size_x4: i32) {
    LOCKED_PROFILE.store(0, Ordering::SeqCst);
    let mut state = PROFILE_STATE.lock().unwrap();
    state.dashboard_settings = [ProfileSettings::default(); XUSER_MAX_COUNT];
    let data_size = (game_defined_data_size_x4 / 4) as usize;

    for i in 0..XUSER_MAX_COUNT {
        if state.profile_data_ptrs[i].is_null() || state.profile_data_sizes[i] < data_size {
            unsafe {
                let new_ptr = malloc(data_size) as *mut u8;
                memset(new_ptr as *mut std::ffi::c_void, 0, data_size);
                state.profile_data_ptrs[i] = new_ptr;
                state.profile_data_sizes[i] = data_size;
            }
        } else {
            unsafe {
                memset(state.profile_data_ptrs[i] as *mut std::ffi::c_void, 0, data_size);
            }
        }

        if data_size >= std::mem::size_of::<ProfileGameSettings>() {
            let settings = unsafe { &mut *(state.profile_data_ptrs[i] as *mut ProfileGameSettings) };
            initialise_default_game_settings(settings);
        }
        ensure_fake_identity_impl(&mut state, i);
    }
}

#[no_mangle]
pub extern "C" fn rust_stub_profile_get_locked_profile() -> i32 {
    LOCKED_PROFILE.load(Ordering::SeqCst)
}

#[no_mangle]
pub extern "C" fn rust_stub_profile_set_locked_profile(prof: i32) {
    LOCKED_PROFILE.store(prof, Ordering::SeqCst);
}

#[no_mangle]
pub extern "C" fn rust_stub_profile_is_signed_in(quadrant: i32) -> bool {
    quadrant == 0
}

#[no_mangle]
pub extern "C" fn rust_stub_profile_is_signed_in_live(prof: i32) -> bool {
    rust_stub_profile_is_signed_in(prof)
}

#[no_mangle]
pub extern "C" fn rust_stub_profile_is_guest(_quadrant: i32) -> bool {
    false
}

#[no_mangle]
pub extern "C" fn rust_stub_profile_query_signin_status() -> bool {
    true
}

#[no_mangle]
pub extern "C" fn rust_stub_profile_get_xuid(pad: i32, out_xuid: *mut u64) {
    if !out_xuid.is_null() {
        let p = if pad >= 0 && (pad as usize) < XUSER_MAX_COUNT { pad as u64 } else { 0 };
        unsafe {
            *out_xuid = FAKE_XUID_BASE + p;
        }
    }
}

#[no_mangle]
pub extern "C" fn rust_stub_profile_are_xuids_equal(xuid1: u64, xuid2: u64) -> bool {
    xuid1 == xuid2
}

#[no_mangle]
pub extern "C" fn rust_stub_profile_xuid_is_guest(_xuid: u64) -> bool {
    false
}

#[no_mangle]
pub extern "C" fn rust_stub_profile_allowed_to_play_multiplayer(_prof: i32) -> bool {
    true
}

#[no_mangle]
pub extern "C" fn rust_stub_profile_get_chat_and_content_restrictions(
    _pad: i32,
    pb_chat_restricted: *mut bool,
    pb_content_restricted: *mut bool,
    pi_age: *mut i32,
) -> bool {
    unsafe {
        if !pb_chat_restricted.is_null() {
            *pb_chat_restricted = false;
        }
        if !pb_content_restricted.is_null() {
            *pb_content_restricted = false;
        }
        if !pi_age.is_null() {
            *pi_age = 18;
        }
    }
    true
}

#[no_mangle]
pub extern "C" fn rust_stub_profile_get_gamertag(pad: i32) -> *mut c_char {
    let p = if pad >= 0 && (pad as usize) < XUSER_MAX_COUNT { pad as usize } else { 0 };
    let mut state = PROFILE_STATE.lock().unwrap();
    ensure_fake_identity_impl(&mut state, p);
    state.gamertags[p].as_mut_ptr()
}

#[no_mangle]
pub extern "C" fn rust_stub_profile_get_display_name(
    pad: i32,
    out_buf: *mut c_char,
    capacity: usize,
) {
    let p = if pad >= 0 && (pad as usize) < XUSER_MAX_COUNT { pad as usize } else { 0 };
    let mut state = PROFILE_STATE.lock().unwrap();
    ensure_fake_identity_impl(&mut state, p);
    let name = &state.display_names[p];
    let bytes = name.as_bytes();
    let copy_len = bytes.len().min(capacity.saturating_sub(1));
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_buf as *mut u8, copy_len);
        *out_buf.add(copy_len) = 0;
    }
}

#[no_mangle]
pub extern "C" fn rust_stub_profile_get_dashboard_profile_settings(
    pad: i32,
) -> *mut ProfileSettings {
    let p = if pad >= 0 && (pad as usize) < XUSER_MAX_COUNT { pad as usize } else { 0 };
    let mut state = PROFILE_STATE.lock().unwrap();
    &mut state.dashboard_settings[p] as *mut ProfileSettings
}

#[no_mangle]
pub extern "C" fn rust_stub_profile_get_game_defined_profile_data(quadrant: i32) -> *mut u8 {
    if quadrant < 0 || (quadrant as usize) >= XUSER_MAX_COUNT {
        return std::ptr::null_mut();
    }
    let p = quadrant as usize;
    let state = PROFILE_STATE.lock().unwrap();
    state.profile_data_ptrs[p]
}

#[no_mangle]
pub extern "C" fn rust_stub_profile_allowed_player_created_content(
    _pad: i32,
    _this_quadrant_only: bool,
    all_allowed: *mut bool,
    friends_allowed: *mut bool,
) {
    unsafe {
        if !all_allowed.is_null() {
            *all_allowed = true;
        }
        if !friends_allowed.is_null() {
            *friends_allowed = true;
        }
    }
}

#[no_mangle]
pub extern "C" fn rust_stub_profile_can_view_player_created_content(
    _pad: i32,
    _this_quadrant_only: bool,
    _p_xuids: *const u64,
    _xuid_count: u32,
) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_game_settings_size() {
        assert_eq!(std::mem::size_of::<ProfileGameSettings>(), 204);
    }

    #[test]
    fn test_initialise_default_game_settings() {
        let mut settings = ProfileGameSettings::default();
        initialise_default_game_settings(&mut settings);

        assert_eq!(settings.uc_menu_sensitivity, 100);
        assert_eq!(settings.uc_interface_opacity, 80);
        assert_ne!(settings.us_bitmask_values & 0x0200, 0);
        assert_ne!(settings.ui_bitmask_values & GAMESETTING_CLOUDS, 0);
        assert_ne!(settings.ui_bitmask_values & GAMESETTING_ONLINE, 0);
        assert_eq!(settings.ui_bitmask_values & GAMESETTING_BEDROCKFOG, 0);
        assert_eq!(settings.uc_tutorial_completion[0], 0xFF);
        assert_eq!(settings.uc_tutorial_completion[1], 0xFF);
        assert_eq!(settings.uc_tutorial_completion[2], 0x0F);
        assert_ne!(settings.uc_tutorial_completion[28] & 1, 0);
    }

    #[test]
    fn test_stub_profile_initialise_and_xuids() {
        rust_stub_profile_initialise(204);

        assert_eq!(rust_stub_profile_get_locked_profile(), 0);
        assert!(rust_stub_profile_is_signed_in(0));
        assert!(!rust_stub_profile_is_signed_in(1));

        let mut xuid = 0u64;
        rust_stub_profile_get_xuid(0, &mut xuid);
        assert_eq!(xuid, FAKE_XUID_BASE);

        let mut xuid1 = 0u64;
        rust_stub_profile_get_xuid(1, &mut xuid1);
        assert_eq!(xuid1, FAKE_XUID_BASE + 1);

        let data_ptr = rust_stub_profile_get_game_defined_profile_data(0);
        assert!(!data_ptr.is_null());
    }
}
