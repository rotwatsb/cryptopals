use utils::{bin_from_b64file, ecb_decrypt};

pub fn prob7() {

    let bin: Vec<u8> = bin_from_b64file("/home/steve/rust/cryptopals/src/7.txt");
    let mut result: Vec<u8> = vec!(0; bin.len());
    let key: Vec<u8> = "YELLOW SUBMARINE".to_string().as_bytes().to_vec();
    ecb_decrypt(&bin[..], &mut result[..], &key[..]);
    if let Ok(r) = String::from_utf8(result) {
        println!("{}", r);
    }
}
