use linops_core::charset;

#[test]
fn test_ascii_is_safe() {
    assert!(charset::is_safe('a'));
    assert!(charset::is_safe('0'));
    assert!(charset::is_safe(' '));
}

#[test]
fn test_box_drawing_is_safe() {
    let horiz = char::from_u32(0x2500).unwrap();
    assert!(charset::is_safe(horiz));
}

#[test]
fn test_braille_is_not_safe() {
    assert!(!charset::is_safe(char::from_u32(0x2800).unwrap()));
}

#[test]
fn test_sanitize_replaces_unsafe() {
    let s = charset::sanitize(char::from_u32(0x2800).unwrap());
    assert!(charset::is_safe(s));
}
