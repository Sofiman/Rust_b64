use b64::decode_b64;

fn decode_to_string(str: &str) -> String {
    String::from_utf8(
        decode_b64(str.as_bytes())
    ).expect("Failed to convert b64 decode result into string")
}

#[test]
#[should_panic]
fn test_decode_invalid() {
    decode_to_string("QW5k*1"); 
}

#[test]
fn test_decode_one_word() {
    let decoded = decode_to_string("SWFt"); 
    assert_eq!("Iam".to_string(), decoded);
}

#[test]
fn test_decode_word_with_one_padding () {
    let decoded = decode_to_string("YW0="); 
    assert_eq!("am".to_string(), decoded);
}

#[test]
fn test_decode_word_with_two_padding() {
    let decoded = decode_to_string("eQ=="); 
    assert_eq!("y".to_string(), decoded);
}

#[test]
fn test_decode_two_words() {
    let decoded = decode_to_string("emlnemFn"); 
    assert_eq!("zigzag".to_string(), decoded);
}

#[test]
fn test_decode_two_words_with_one_padding() {
    let decoded = decode_to_string("Q2F0RG9nMjE="); 
    assert_eq!("CatDog21".to_string(), decoded);
}
