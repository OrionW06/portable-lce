#[no_mangle]
pub extern "C" fn rust_stub_leaderboard_open_session() -> bool {
    true
}

#[no_mangle]
pub extern "C" fn rust_stub_leaderboard_write_stats() -> bool {
    false
}

#[no_mangle]
pub extern "C" fn rust_stub_leaderboard_read_stats() -> bool {
    false
}

#[no_mangle]
pub extern "C" fn rust_stub_leaderboard_is_idle() -> bool {
    true
}
