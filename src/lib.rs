#[allow(dead_code)]

/* ENCODING BASE64 */

const B64_TABLE: &[u8] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();

pub fn encode_b64(in_buf: &[u8]) -> Vec<u8> {
    let total_len: usize = in_buf.len();
    let len = (1 + (total_len - 1) / 3) * 4;
    // Calculate target buffer len by rounding up without using floating-point

    let mut out_vec = vec![0u8; len];
    let mut word: u64 = 0;
    let mut pos: usize = 0;

    let out_buf: &mut [u8] = out_vec.as_mut_slice();

    #[allow(clippy::needless_range_loop)]
    for i in 0..total_len {
        word = (word << 8) + in_buf[i] as u64;
        if i % 3 == 2 { // if we have 4 bytes worth of data in word
            out_buf[pos + 3] = B64_TABLE[(word         & 0b111111) as usize];
            out_buf[pos + 2] = B64_TABLE[((word >>  6) & 0b111111) as usize];
            out_buf[pos + 1] = B64_TABLE[((word >> 12) & 0b111111) as usize];
            out_buf[pos    ] = B64_TABLE[((word >> 18) & 0b111111) as usize];

            pos += 4;
            word = 0;
        }
    }
    let remaining = total_len % 3;
    if remaining > 0 {
        // add remaining bytes and add padding
        word <<= (3 - remaining) * 8; // move the remaining bytes to the MSB
        let mut n;
        for i in 0..4 {
            if i <= remaining {
                n = ((word >> (18-i*6)) & 0b111111) as usize;
                out_buf[pos + i] = B64_TABLE[n];
            } else {
                out_buf[pos + i] = b'=';
            }
        }
    }
    out_vec 
}

/* DECODING BASE64 */

const B64_INV_TABLE: [u8; 255] = 
[
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    62, // +
    0xFF, 0xFF, 0xFF, // , - .
    63, // /
    52, 53, 54, 55, 56, 57, 58, 59, 60, 61, // 0-9
    0xFF, 0xFF, 0xFF,  // : - <
    0, // =
    0xFF, 0xFF, 0xFF, // > - @
    00,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, // A - Z
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // [ - `
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, // a - z
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF
]; // Table used to convert chars into indexes

pub fn decode_b64(in_buf: &[u8]) -> Vec<u8> {
    let len = in_buf.len();
/*
    let mut padding: usize = len-3;
    while padding < len && in_buf[padding] == b'=' {
        padding += 1
    }
    padding = len-padding;

    let expected_len = (1 + (len - 1) / 4) * 3 - padding;
*/
    let mut out_vec = vec![0u8; (len / 4) * 3];
    let out_buf = out_vec.as_mut_slice();

    let mut pos: usize = 0;
    let mut word: u64 = 0;

    let mut chr: u8;

    #[allow(clippy::needless_range_loop)]
    for i in 0..len {
        chr = B64_INV_TABLE[in_buf[i] as usize];
        if chr == 0xFF {
            panic!("Invalid base64 character at index {}", i);
        }
        word = (word << 6) + chr as u64;
        if i % 4 == 3 {
            for j in 0..3 {
                chr = (word & 0xFF) as u8;
                if chr == 0 {
                    word >>= 8;
                    continue;
                }
                out_buf[pos + 2 - j] = chr;
                word >>= 8;
            }
            pos += 3;
            word = 0;
        }
    }

    out_vec
}
