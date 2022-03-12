use std::io;
use std::time::Instant;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();
    
    let enc_start = Instant::now();
    
    let output = String::from_utf8(b64::encode_b64(input.as_bytes())).unwrap();

    let enc_time = enc_start.elapsed();
    println!("encoded in {:?}", enc_time);
    
    let dec_start = Instant::now();
    
    let decoded = String::from_utf8(b64::decode_b64(output.as_bytes())).unwrap();
    
    let dec_time = dec_start.elapsed();

    //encoder::encode_b64(input.as_bytes());
    println!("decoded in {:?}", dec_time);
    assert_eq!(input, decoded);
}
