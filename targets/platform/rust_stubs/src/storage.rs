use std::ffi::c_char;

pub fn crc(buf: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &b in buf {
        crc ^= b as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg() & 0xEDB8_8320;
            crc = (crc >> 1) ^ mask;
        }
    }
    !crc
}

#[derive(Default, Debug, Clone)]
pub struct Subfile {
    pub region_index: i32,
    pub data: Vec<u8>,
}

#[derive(Default, Debug)]
pub struct StubStorage {
    pub save_disabled: bool,
    pub save_unique_number: i32,
    pub subfiles: Vec<Subfile>,
}

impl StubStorage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn crc(&self, buf: &[u8]) -> u32 {
        crc(buf)
    }

    pub fn request_message_box(&self) -> i32 {
        1 // EMessage_ResultAccept
    }

    pub fn get_message_box_result(&self) -> i32 {
        0 // EMessage_Undefined
    }

    pub fn set_save_device(&self) -> bool {
        true
    }

    pub fn get_save_unique_number(&self) -> (i32, bool) {
        (self.save_unique_number, true)
    }

    pub fn get_save_disabled(&self) -> bool {
        self.save_disabled
    }

    pub fn get_save_size(&self) -> u32 {
        0
    }

    pub fn get_save_device_selected(&self, _pad: u32) -> bool {
        true
    }

    pub fn does_save_exist(&self) -> (bool, i32) {
        (false, 0) // (exists = false, ESaveGame_Idle = 0)
    }

    pub fn enough_space_for_min_save_game(&self) -> bool {
        true
    }

    pub fn get_dlc_offers(&self) -> i32 {
        0 // EDLC_NoOffers
    }

    pub fn add_subfile(&mut self, region_index: i32) -> usize {
        let idx = self.subfiles.len();
        self.subfiles.push(Subfile {
            region_index,
            data: Vec::new(),
        });
        idx
    }

    pub fn get_subfile_count(&self) -> usize {
        self.subfiles.len()
    }

    pub fn reset_subfiles(&mut self) {
        self.subfiles.clear();
    }
}

// C FFI exports
#[no_mangle]
pub extern "C" fn rust_stub_storage_crc(buf: *const u8, len: usize) -> u32 {
    if buf.is_null() || len == 0 {
        return 0;
    }
    let slice = unsafe { std::slice::from_raw_parts(buf, len) };
    crc(slice)
}

#[no_mangle]
pub extern "C" fn rust_stub_storage_request_message_box() -> i32 {
    1 // EMessage_ResultAccept
}

#[no_mangle]
pub extern "C" fn rust_stub_storage_get_message_box_result() -> i32 {
    0 // EMessage_Undefined
}

#[no_mangle]
pub extern "C" fn rust_stub_storage_set_save_device() -> bool {
    true
}

#[no_mangle]
pub extern "C" fn rust_stub_storage_get_save_unique_number(out_val: *mut i32) -> bool {
    if !out_val.is_null() {
        unsafe {
            *out_val = 0;
        }
    }
    true
}

#[no_mangle]
pub extern "C" fn rust_stub_storage_get_save_unique_filename(out_name: *mut c_char) -> bool {
    if !out_name.is_null() {
        unsafe {
            *out_name = 0;
        }
    }
    true
}

#[no_mangle]
pub extern "C" fn rust_stub_storage_get_save_disabled() -> bool {
    false
}

#[no_mangle]
pub extern "C" fn rust_stub_storage_get_save_size() -> u32 {
    0
}

extern "C" {
    fn malloc(size: usize) -> *mut std::ffi::c_void;
    fn free(ptr: *mut std::ffi::c_void);
}

#[no_mangle]
pub extern "C" fn rust_stub_storage_allocate_save_data(bytes: u32) -> *mut std::ffi::c_void {
    if bytes == 0 {
        return std::ptr::null_mut();
    }
    unsafe { malloc(bytes as usize) }
}

#[no_mangle]
pub extern "C" fn rust_stub_storage_free_save_data(ptr: *mut std::ffi::c_void) {
    if !ptr.is_null() {
        unsafe { free(ptr) };
    }
}

#[no_mangle]
pub extern "C" fn rust_stub_storage_get_save_device_selected(_pad: u32) -> bool {
    true
}

#[no_mangle]
pub extern "C" fn rust_stub_storage_does_save_exist(out_exists: *mut bool) -> i32 {
    if !out_exists.is_null() {
        unsafe {
            *out_exists = false;
        }
    }
    0 // ESaveGame_Idle
}

#[no_mangle]
pub extern "C" fn rust_stub_storage_enough_space_for_min_save_game() -> bool {
    true
}

#[no_mangle]
pub extern "C" fn rust_stub_storage_get_dlc_offers() -> i32 {
    0 // EDLC_NoOffers
}

#[no_mangle]
pub extern "C" fn rust_stub_storage_get_installed_dlc(
    pad: i32,
    callback: Option<extern "C" fn(i32, i32) -> i32>,
) -> i32 {
    if let Some(cb) = callback {
        cb(0, pad);
    }
    0 // EDLC_NoInstalledDLC
}

#[no_mangle]
pub extern "C" fn rust_stub_storage_read_tms_file() -> i32 {
    0 // ETMSStatus_Fail
}

#[no_mangle]
pub extern "C" fn rust_stub_storage_write_tms_file() -> bool {
    false
}

#[no_mangle]
pub extern "C" fn rust_stub_storage_delete_tms_file() -> bool {
    false
}

#[no_mangle]
pub extern "C" fn rust_stub_storage_tmspp_read_file() -> i32 {
    0 // ETMSStatus_Fail
}

#[no_mangle]
pub extern "C" fn rust_stub_storage_save_subfiles(
    callback: Option<extern "C" fn(bool) -> i32>,
) {
    if let Some(cb) = callback {
        cb(true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc32() {
        let data = b"123456789";
        // Standard CRC32 algorithm for "123456789" is 0xCBF43926
        assert_eq!(crc(data), 0xCBF43926);
    }

    #[test]
    fn test_stub_storage_subfiles() {
        let mut storage = StubStorage::new();
        assert_eq!(storage.get_subfile_count(), 0);

        let idx0 = storage.add_subfile(10);
        let idx1 = storage.add_subfile(20);

        assert_eq!(idx0, 0);
        assert_eq!(idx1, 1);
        assert_eq!(storage.get_subfile_count(), 2);

        storage.reset_subfiles();
        assert_eq!(storage.get_subfile_count(), 0);
    }
}
