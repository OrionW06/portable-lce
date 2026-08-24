use std::ffi::c_char;

#[derive(Default, Debug)]
pub struct AppConfig {
    pub width: i32,
    pub height: i32,
    pub fullscreen: bool,
}

impl AppConfig {
    pub fn parse_args(args: &[String]) -> Self {
        let mut config = AppConfig::default();
        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--fullscreen" => config.fullscreen = true,
                "--width" if i + 1 < args.len() => {
                    if let Ok(w) = args[i + 1].parse() {
                        config.width = w;
                    }
                    i += 1;
                }
                "--height" if i + 1 < args.len() => {
                    if let Ok(h) = args[i + 1].parse() {
                        config.height = h;
                    }
                    i += 1;
                }
                _ => {}
            }
            i += 1;
        }
        config
    }
}

pub fn define_joypad_action_mappings() {
    // Joypad action mappings definition
}

pub fn run_app_main(args: Vec<String>) -> i32 {
    let _config = AppConfig::parse_args(&args);
    define_joypad_action_mappings();
    0
}

#[no_mangle]
pub extern "C" fn rust_app_main(argc: i32, argv: *const *const c_char) -> i32 {
    let mut args = Vec::new();
    if !argv.is_null() && argc > 0 {
        for i in 0..argc {
            unsafe {
                let ptr = *argv.add(i as usize);
                if !ptr.is_null() {
                    let cstr = std::ffi::CStr::from_ptr(ptr);
                    args.push(cstr.to_string_lossy().into_owned());
                }
            }
        }
    }
    run_app_main(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_config_args() {
        let args = vec![
            "Minecraft.Client".to_string(),
            "--width".to_string(),
            "1280".to_string(),
            "--height".to_string(),
            "720".to_string(),
            "--fullscreen".to_string(),
        ];
        let cfg = AppConfig::parse_args(&args);
        assert_eq!(cfg.width, 1280);
        assert_eq!(cfg.height, 720);
        assert!(cfg.fullscreen);
    }
}
