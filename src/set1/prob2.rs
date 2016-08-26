use std::cmp;

use set1::prob1::{from_hex_string, to_base, HEX_MAP};
    
pub fn prob2() {
    let hex1 = "1c0111001f010100061a024b53535009181c".to_string();
    let hex2 = "686974207468652062756c6c277320657965".to_string();

    let new_hex: String = to_base(&xor_vecs(&from_hex_string(&hex1)[..],
                                            &from_hex_string(&hex2)[..]), &HEX_MAP);

    println!("{:?}", new_hex);
}

pub fn xor_vecs(v1: &[u8], v2: &[u8]) -> Vec<u8> {
    let mut v3: Vec<u8> = Vec::new();
    let l = cmp::min(v1.len(), v2.len());

    for i in 0..l {
        v3.push(v1[i] ^ v2[i]);
    }
    
    v3
}


