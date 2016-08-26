use utils::{bin_from_b64file, cbc_decrypt};

pub fn prob10() {
    
    let bin: Vec<u8> =
        bin_from_b64file("/home/steve/rust/cryptopals/src/10.txt");
    let iv: Vec<u8> = vec!(0; 16);
    let key: Vec<u8> = "YELLOW SUBMARINE".to_string().as_bytes().to_vec();

    /*
    let mut bin2: Vec<u8> =
        bin_from_b64file("/home/steve/rust/cryptopals/src/7.txt");
    
    let mut decrypted_bin2: Vec<u8> = vec!(0; bin2.len());
    ecb_decrypt(&bin2[..], &mut decrypted_bin2[..], &key[..]);
    if let Ok(s) = String::from_utf8(decrypted_bin2.clone()) {
        println!("{}", s);
    }

    let reencrypted: Vec<u8> = cbc_encrypt(&mut decrypted_bin2, &iv, &key);
    println!("{}", reencrypted.clone().to_base64(STANDARD));
    */
    let decrypted: Vec<u8> = cbc_decrypt(&bin, &iv, &key);
    if let Ok(s) = String::from_utf8(decrypted) {
        println!("{}", s);
    }
}
