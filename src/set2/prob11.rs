use rand::{OsRng, Rng};

use utils::{byte_repeats_exist, pad_to, ecb_encrypt, cbc_encrypt};

pub fn prob11() {
    ecb_or_cbc_oracle(&rand_encrypt_ecb_or_cbc);
}

fn ecb_or_cbc_oracle(encryptor: &Fn(&mut Vec<u8>) -> Vec<u8>) {
    let mut bin: Vec<u8> = "HEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHE".to_string().as_bytes().to_vec();
    let encrypted: Vec<u8> = encryptor(&mut bin);
    if byte_repeats_exist(&encrypted, 16) {
        println!("ECB!");
    } else { println!("CBC!"); }
}

fn rand_encrypt_ecb_or_cbc(bin: &mut Vec<u8>) -> Vec<u8> {

    let mut g: OsRng = OsRng::new().unwrap();
    let key: Vec<u8> = g.gen_iter().take(16).collect::<Vec<u8>>();
    let iv: Vec<u8> = g.gen_iter().take(16).collect::<Vec<u8>>();
    let p: usize = (g.next_f32() * 5.0) as usize + 5;
    let a: usize = (g.next_f32() * 5.0) as usize + 5;
    let prepend: Vec<u8> = g.gen_iter().take(p).collect::<Vec<u8>>();
    let append: Vec<u8> = g.gen_iter().take(a).collect::<Vec<u8>>();

    bin.extend(append.iter());
    for e in prepend { bin.insert(0, e); }

    pad_to(bin, 16);    
    let do_ecb: bool = g.next_f32() < 0.5;
    
    if do_ecb {
        let mut result: Vec<u8> = vec!(0; bin.len());
        ecb_encrypt(&bin[..], &mut result[..], &key[..]);
        result
    } else {
        cbc_encrypt(bin, &iv, &key)
    }
}



