pub fn to_lower(s: &str) -> String {
    s.to_lowercase()
}

pub fn trim_string(s: &str) -> String {
    s.trim().to_string()
}

pub fn replace_all(s: &str, replace: &str, with: &str) -> String {
    s.replace(replace, with)
}

pub fn equals_ignore_case(a: &str, b: &str) -> bool {
    a.eq_ignore_ascii_case(b)
}

pub fn conv_string_to_wstring(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_helpers() {
        assert_eq!(to_lower("Hello WORLD"), "hello world");
        assert_eq!(trim_string("   foo bar  \n"), "foo bar");
        assert_eq!(replace_all("foo bar foo", "foo", "baz"), "baz bar baz");
        assert!(equals_ignore_case("Minecraft", "MINECRAFT"));
        assert!(!equals_ignore_case("Minecraft", "Other"));

        let wide = conv_string_to_wstring("hello");
        assert_eq!(wide, vec![104, 101, 108, 108, 111]);
    }
}
