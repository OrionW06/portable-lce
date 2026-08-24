pub const VERSION_STRING: &str = "1.6.4";
pub const NETWORK_PROTOCOL_VERSION: i32 = 78;
pub const INGAME_DEBUG_OUTPUT: bool = false;
pub const WORLD_RESOLUTION: i32 = 16;
pub const MAX_CHAT_LENGTH: i32 = 100;
pub const TEXTURE_LIGHTING: bool = true;
pub const TICKS_PER_SECOND: i32 = 20;
pub const FULLBRIGHT_LIGHTVALUE: i32 = (15 << 20) | (15 << 4);
pub const ILLEGAL_FILE_CHARACTERS: [char; 15] = [
    '/', '\n', '\r', '\t', '\0', '\x0C', '`', '?',
    '*', '\\', '<', '>', '|', '"', ':',
];

pub const CLIENT_VERSION_STRING: &str = "1.6.4";
pub const DEADMAU5_CAMERA_CHEATS: bool = false;
pub const IS_DEMO_VERSION: bool = false;

pub fn read_acceptable_chars() -> &'static str {
    " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~ ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜø£Ø×ƒáíóúñÑªº¿®¬½¼¡«»ã"
}

pub fn is_allowed_chat_character(_ch: char) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(VERSION_STRING, "1.6.4");
        assert_eq!(NETWORK_PROTOCOL_VERSION, 78);
        assert_eq!(FULLBRIGHT_LIGHTVALUE, 15728880);
        assert!(is_allowed_chat_character('a'));
    }
}
