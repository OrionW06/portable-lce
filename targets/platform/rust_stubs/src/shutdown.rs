use std::sync::atomic::{AtomicBool, Ordering};

static SHUTDOWN_REQUESTED: AtomicBool = AtomicBool::new(false);
static RESTART_REQUESTED: AtomicBool = AtomicBool::new(false);

#[no_mangle]
pub extern "C" fn rust_shutdown_manager_request_shutdown() {
    SHUTDOWN_REQUESTED.store(true, Ordering::SeqCst);
}

#[no_mangle]
pub extern "C" fn rust_shutdown_manager_request_restart() {
    RESTART_REQUESTED.store(true, Ordering::SeqCst);
}

#[no_mangle]
pub extern "C" fn rust_shutdown_manager_is_shutdown_requested() -> bool {
    SHUTDOWN_REQUESTED.load(Ordering::SeqCst)
}

#[no_mangle]
pub extern "C" fn rust_shutdown_manager_is_restart_requested() -> bool {
    RESTART_REQUESTED.load(Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shutdown_manager_state() {
        assert!(!rust_shutdown_manager_is_shutdown_requested());
        assert!(!rust_shutdown_manager_is_restart_requested());

        rust_shutdown_manager_request_shutdown();
        assert!(rust_shutdown_manager_is_shutdown_requested());

        rust_shutdown_manager_request_restart();
        assert!(rust_shutdown_manager_is_restart_requested());
    }
}
