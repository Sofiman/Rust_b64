use b64::encode_b64;

fn b64_to_str(str: &str) -> String {
    String::from_utf8(
        encode_b64(str.to_string().as_bytes())
    ).expect("Failed to convert b64 result back to string")
}

#[test]
fn test_encode_one_word() {
    let encoded = b64_to_str("And");
    assert_eq!("QW5k".to_string(), encoded);
}

#[test]
fn test_encode_with_two_padding() {
    let encoded = b64_to_str("g");
    assert_eq!("Zw==".to_string(), encoded);
}

#[test]
fn test_encode_two_words() {
    let encoded = b64_to_str("OneTwo");
    assert_eq!("T25lVHdv".to_string(), encoded);
}

#[test]
fn test_encode_word_with_two_padding() {
    let encoded = b64_to_str("Andg");
    assert_eq!("QW5kZw==".to_string(), encoded);
}

#[test]
fn test_encode_word_with_one_padding() {
    let encoded = b64_to_str("Stack");
    assert_eq!("U3RhY2s=".to_string(), encoded);
}

#[test]
fn test_encode_long_word() {
    let encoded = b64_to_str("Lorem ipsum dolor sit amet, consectetur adipiscing elit.");
    assert_eq!("TG9yZW0gaXBzdW0gZG9sb3Igc2l0IGFtZXQsIGNvbnNlY3RldHVyIGFkaXBpc2NpbmcgZWxpdC4=".to_string(), encoded);
}
