use rustc_serialize::hex::{ToHex};

use std::cmp;

pub fn prob5() {
    let seq_str: String = "ICE".to_string();
    let seq_bytes = seq_str.as_bytes();

    println!("{:?}", seq_bytes);
    
    let to_encode_str: String = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".to_string();

    let to_encode_bytes = to_encode_str.as_bytes();

    let mut encoded_bytes: Vec<u8> = vec!(0; to_encode_bytes.len());
    repeating_seq_xor(to_encode_bytes, &mut encoded_bytes, &seq_bytes);
    println!("{}", encoded_bytes[..].to_hex());
}

pub fn repeating_seq_xor(to_encode: &[u8], result: &mut Vec<u8>, seq: &[u8]) {
    for i in 0 .. to_encode.len() {
        result[i] = seq[i % seq.len()] ^ to_encode[i];
    }
}


