use std::ffi::c_char;

// Gamepad virtual key constants
pub const VK_PAD_A: u32 = 0x5800;
pub const VK_PAD_B: u32 = 0x5801;
pub const VK_PAD_X: u32 = 0x5802;
pub const VK_PAD_Y: u32 = 0x5803;
pub const VK_PAD_RSHOULDER: u32 = 0x5804;
pub const VK_PAD_LSHOULDER: u32 = 0x5805;
pub const VK_PAD_LTRIGGER: u32 = 0x5806;
pub const VK_PAD_RTRIGGER: u32 = 0x5807;

pub const VK_PAD_DPAD_UP: u32 = 0x5810;
pub const VK_PAD_DPAD_DOWN: u32 = 0x5811;
pub const VK_PAD_DPAD_LEFT: u32 = 0x5812;
pub const VK_PAD_DPAD_RIGHT: u32 = 0x5813;
pub const VK_PAD_START: u32 = 0x5814;
pub const VK_PAD_BACK: u32 = 0x5815;
pub const VK_PAD_LTHUMB_PRESS: u32 = 0x5816;
pub const VK_PAD_RTHUMB_PRESS: u32 = 0x5817;

// Language constants
pub const XC_LANGUAGE_ENGLISH: u32 = 0x01;
pub const XC_LANGUAGE_JAPANESE: u32 = 0x02;
pub const XC_LANGUAGE_GERMAN: u32 = 0x03;
pub const XC_LANGUAGE_FRENCH: u32 = 0x04;
pub const XC_LANGUAGE_SPANISH: u32 = 0x05;
pub const XC_LANGUAGE_ITALIAN: u32 = 0x06;
pub const XC_LANGUAGE_KOREAN: u32 = 0x07;
pub const XC_LANGUAGE_TCHINESE: u32 = 0x08;

// System notification constants
pub const XN_SYS_SIGNINCHANGED: u32 = 0;
pub const XN_SYS_INPUTDEVICESCHANGED: u32 = 1;
pub const XN_LIVE_CONTENT_INSTALLED: u32 = 2;
pub const XN_SYS_STORAGEDEVICESCHANGED: u32 = 3;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct CXuiStringTable {
    loaded_id: Option<String>,
}

impl CXuiStringTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lookup_id<'a>(&self, sz_id: &'a str) -> &'a str {
        sz_id
    }

    pub fn lookup_index(&self, _n_index: u32) -> &'static str {
        "String"
    }

    pub fn clear(&mut self) {
        self.loaded_id = None;
    }

    pub fn load(&mut self, sz_id: &str) -> i32 {
        self.loaded_id = Some(sz_id.to_string());
        0
    }
}

pub fn is_equal_xuid(a: u64, b: u64) -> bool {
    a == b
}

pub fn xuser_get_signin_info(_dw_user_index: u32, _dw_flags: u32) -> u32 {
    0
}

pub fn xget_language() -> u32 {
    XC_LANGUAGE_ENGLISH
}

pub fn xget_locale() -> u32 {
    0
}

pub fn xenable_guest_signin(_f_enable: bool) -> u32 {
    0
}

// C FFI exports
#[no_mangle]
pub extern "C" fn rust_is_equal_xuid(a: u64, b: u64) -> bool {
    is_equal_xuid(a, b)
}

#[no_mangle]
pub extern "C" fn rust_xuser_get_signin_info(
    dw_user_index: u32,
    dw_flags: u32,
    _p_signin_info: *mut std::ffi::c_void,
) -> u32 {
    xuser_get_signin_info(dw_user_index, dw_flags)
}

#[no_mangle]
pub extern "C" fn rust_cxui_string_table_lookup_id(sz_id: *const c_char) -> *const c_char {
    sz_id
}

static STRING_CONST: &[u8] = b"String\0";

#[no_mangle]
pub extern "C" fn rust_cxui_string_table_lookup_index(_n_index: u32) -> *const c_char {
    STRING_CONST.as_ptr() as *const c_char
}

#[no_mangle]
pub extern "C" fn rust_cxui_string_table_clear() {}

#[no_mangle]
pub extern "C" fn rust_cxui_string_table_load(_sz_id: *const c_char) -> i32 {
    0
}

#[no_mangle]
pub extern "C" fn rust_xget_language() -> u32 {
    xget_language()
}

#[no_mangle]
pub extern "C" fn rust_xget_locale() -> u32 {
    xget_locale()
}

#[no_mangle]
pub extern "C" fn rust_xenable_guest_signin(f_enable: bool) -> u32 {
    xenable_guest_signin(f_enable)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xuid_equality() {
        assert!(is_equal_xuid(12345, 12345));
        assert!(!is_equal_xuid(12345, 67890));
    }

    #[test]
    fn test_string_table() {
        let mut table = CXuiStringTable::new();
        assert_eq!(table.lookup_id("IDS_TEST"), "IDS_TEST");
        assert_eq!(table.lookup_index(42), "String");
        assert_eq!(table.load("IDS_TEST"), 0);
        table.clear();
    }

    #[test]
    fn test_language_and_locale() {
        assert_eq!(xget_language(), XC_LANGUAGE_ENGLISH);
        assert_eq!(xget_locale(), 0);
        assert_eq!(xenable_guest_signin(true), 0);
    }
}
