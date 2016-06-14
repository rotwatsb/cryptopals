extern crate rustc_serialize;
extern crate rand;
extern crate crypto;

use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::cmp::Ordering;
use std::collections::BTreeMap;

use rand::{OsRng, Rng};

use rustc_serialize::base64::{ToBase64, FromBase64, STANDARD};
use rustc_serialize::hex::{ToHex, FromHex};

use crypto::aes;
use crypto::aessafe::AesSafe128Decryptor;
use crypto::symmetriccipher::{Decryptor, BlockDecryptor, Encryptor, BlockEncryptor};
use crypto::blockmodes::{NoPadding, EcbDecryptor, EcbEncryptor};
use crypto::buffer::{RefReadBuffer, RefWriteBuffer, ReadBuffer, WriteBuffer};

const BIN_MAP: [char; 2] = ['0', '1'];

const HEX_MAP: [char; 16] =
    ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd',
     'e', 'f'];

const B64_MAP: [char; 64] =
    ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
     'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
     'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
     'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3',
     '4', '5', '6', '7', '8', '9', '+', '/'];

const DEBUG: bool = false;

fn main() {
    //prob1();
    //prob2();
    //prob3();
    //prob4();
    //prob5();
    //prob6();
    //prob7a();
    //prob8();
    //prob9();
    //prob10();
    //prob11();
    //test();
}

fn prob12() {
    let mut g: OsRng = OsRng::new().unwrap();
    let mut key: Vec<u8> = g.gen_iter().take(16).collect::<Vec<u8>>();

    
}

fn prob11() {
    ecb_or_cbc_oracle(&rand_encrypt_ecb_or_cbc);
}

fn ecb_or_cbc_oracle(encryptor: &Fn(&mut Vec<u8>) -> Vec<u8>) {
    let mut bin: Vec<u8> = "HEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHEHE".to_string().as_bytes().to_vec();
    let mut encrypted: Vec<u8> = encryptor(&mut bin);
    if byte_repeats_exist(&encrypted, 16) {
        println!("ECB!");
    } else { println!("CBC!"); }
}

fn rand_encrypt_ecb_or_cbc(bin: &mut Vec<u8>) -> Vec<u8> {

    let mut g: OsRng = OsRng::new().unwrap();
    let mut key: Vec<u8> = g.gen_iter().take(16).collect::<Vec<u8>>();
    let mut iv: Vec<u8> = g.gen_iter().take(16).collect::<Vec<u8>>();
    let p: usize = (g.next_f32() * 5.0) as usize + 5;
    let a: usize = (g.next_f32() * 5.0) as usize + 5;
    let mut prepend: Vec<u8> = g.gen_iter().take(p).collect::<Vec<u8>>();
    let mut append: Vec<u8> = g.gen_iter().take(a).collect::<Vec<u8>>();

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

fn prob10() {
    
    let mut bin: Vec<u8> =
        bin_from_b64file("/home/steve/rust/cryptopals/src/10.txt");
    let mut iv: Vec<u8> = vec!(0; 16);
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
    let mut decrypted: Vec<u8> = cbc_decrypt(&bin, &iv, &key);
    if let Ok(s) = String::from_utf8(decrypted) {
        println!("{}", s);
    }
}

fn cbc_encrypt(bin: &mut Vec<u8>, iv: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {

    let chunksize: usize = iv.len();
    pad_to(bin, chunksize);
    
    let mut bin2: Vec<u8> = bin.clone();
    let mut vec_out: Vec<u8> = vec!(0; bin.len());
    let mut l: usize = bin2.len();

    set_xor_slice(&bin[0..chunksize], &iv[..], &mut bin2[0..chunksize]);
   
    for i in 0 .. l / chunksize {
        if i > 0 {
            set_xor_slice(&bin[i * chunksize .. (i + 1) * chunksize],
                           &vec_out[(i - 1) * chunksize .. i * chunksize],
                           &mut bin2[i * chunksize .. (i + 1) * chunksize]);

        }
        ecb_encrypt(&bin2[i * chunksize .. (i + 1) * chunksize],
                    &mut vec_out[i * chunksize .. (i + 1) * chunksize],
                    &key[..]);
    }

    vec_out
}

fn cbc_decrypt(bin: &Vec<u8>, iv: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {

    let chunksize: usize = iv.len();

    let mut vec_out: Vec<u8> = vec!(0; bin.len());
    let mut vec_out2: Vec<u8> = vec_out.clone();
    let mut l: usize = bin.len();

       for i in 0 .. l / chunksize {
        ecb_decrypt(&bin[i * chunksize .. (i + 1) * chunksize],
                    &mut vec_out[i * chunksize .. (i + 1) * chunksize],
                    &key[..]);
           if i > 0 {
            set_xor_slice(&bin[(i - 1) * chunksize .. i * chunksize],
                          &vec_out[i * chunksize .. (i + 1) * chunksize],
                          &mut vec_out2[i * chunksize .. (i + 1) * chunksize]);
        } else {
            set_xor_slice(&iv[..],
                          &vec_out[i * chunksize .. (i + 1) * chunksize],
                          &mut vec_out2[i * chunksize .. (i + 1) * chunksize]);
        }
           
    }

    vec_out2
}

fn prob9() {
    let mut x: Vec<u8> = "YELLOW SUBMARINE".to_string().as_bytes().to_vec();
    pad_to(&mut x, 15);
    println!("{:?}", String::from_utf8(x).unwrap());
}

fn pad_to(to_pad: &mut Vec<u8>, block_size: usize) {
    let to_append: usize = (block_size - (to_pad.len() % block_size)) % block_size;

    for i in 0 .. to_append {
        to_pad.push(to_append as u8);
    }
}

fn prob8() {
    if let Ok(t) = File::open("/home/steve/rust/cryptopals/src/8.txt") {
        let reader = BufReader::new(t);
        for line_res in reader.lines() {
            if let Ok(line) = line_res {
                if byte_repeats_exist(&line.as_bytes().to_vec(), 16) {
                    println!("{}", line);
                }
            }
        }
    }
}

fn byte_repeats_exist(data: &Vec<u8>, n: usize) -> bool {

    let mut block_map: BTreeMap<&[u8], u32> = BTreeMap::new();

    for i in 0 .. data.len() / n {
        let mut x: &[u8] = &data[i*n .. std::cmp::min(data.len(), i*n+n)];
        if !block_map.contains_key(&x) {
            block_map.insert(x, 1);
        }
        else { return true; }
    }

    false
}

fn ecb_decrypt(slice_in: &[u8], slice_out: &mut [u8], key: &[u8]) {
    
    let mut boxed_dec: Box<Decryptor> =
        aes::ecb_decryptor(aes::KeySize::KeySize128, &key[..], NoPadding);

    let mut buff_in = RefReadBuffer::new(slice_in);
    let mut buff_out = RefWriteBuffer::new(slice_out);
    
    (*boxed_dec).decrypt(&mut buff_in, &mut buff_out, true);
    
}

fn ecb_encrypt(slice_in: &[u8], slice_out: &mut [u8], key: &[u8]) {
    
    let mut boxed_enc: Box<Encryptor> =
        aes::ecb_encryptor(aes::KeySize::KeySize128, &key[..], NoPadding);

    let mut buff_in = RefReadBuffer::new(slice_in);
    let mut buff_out = RefWriteBuffer::new(slice_out);
    
    (*boxed_enc).encrypt(&mut buff_in, &mut buff_out, true);
}

fn bin_from_b64file(f: &str) -> Vec<u8> {
    let mut cipher_text = String::new();

    if let Ok(t) = File::open(f) {
        let reader = BufReader::new(t);
        for line_res in reader.lines() {
            if let Ok(line) = line_res {
                cipher_text = cipher_text + &line;
            }
        }
    }

    cipher_text.from_base64().unwrap()
}

fn prob7a() {

    let mut bin: Vec<u8> = bin_from_b64file("/home/steve/rust/cryptopals/src/7.txt");
    let mut result: Vec<u8> = vec!(0; bin.len());
    let key: Vec<u8> = "YELLOW SUBMARINE".to_string().as_bytes().to_vec();
    ecb_decrypt(&bin[..], &mut result[..], &key[..]);
    if let Ok(r) = String::from_utf8(result) {
        println!("{}", r);
    }
}

fn prob6() {
    let bin: Vec<u8> = "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"
        .to_string().from_base64().unwrap();

    let keysize_min: usize = 2;
    let keysize_max: usize = if bin.len() / 4 > 40 { 40 } else { bin.len() / 4 };
    let mut ham_dist: f64 = 0.0;
    let mut min_ham: f64 = std::f64::MAX;
    let mut keysizes: Vec<(usize, f64)> = Vec::new();
    
    for i in keysize_min .. keysize_max + 1 {
        ham_dist = hamming_distance(&bin[0..i], &bin[i..(2*i)]);
        ham_dist += hamming_distance(&bin[i..(2*i)], &bin[(2*i)..(3*i)]);
        ham_dist += hamming_distance(&bin[(2*i)..(3*i)], &bin[(3*i)..(4*i)]);
        ham_dist += hamming_distance(&bin[(3*i)..(4*i)], &bin[0..i]);
        keysizes.push((i, ham_dist / (4.0 * i as f64)));
    }

    keysizes.sort_by(|&(s1, d1), &(s2, d2)| { if d1 < d2 { Ordering::Less }
                                              else if d1 == d2 { Ordering::Equal }
                                              else { Ordering::Greater } });
    if DEBUG {
        println!("keysizes: {:?}", keysizes); 
        println!("raw binary: {:?}", bin);
        println!("raw as hex: {:?}", bin.to_hex());
    }

    let mut keysize: usize = 0;
    for k in 0 .. 4  {
        keysize = keysizes[k].0;
        if DEBUG {
            println!("");
            println!("key size: {}", keysize);
        }
        let mut seq: Vec<u8> = Vec::new();

        let mut transposed_blocks: Vec<Vec<u8>> = Vec::new();
        for i in 0 .. keysize { transposed_blocks.push(Vec::new()); }
        for i in 0 .. bin.len() { transposed_blocks[i % keysize].push(bin[i]); }

        for i in 0 .. keysize {
            if DEBUG { println!("\ni: {}\n{:?}", i, transposed_blocks[i]); }
            let (best_decode, best_key, max_score) =
                decode_repeating_char_xor(&transposed_blocks[i]);
            seq.push(best_key);
        }

        let mut result: Vec<u8> = vec!(0; bin.len());
        repeating_seq_xor(&bin[..], &mut result, &seq[..]);
        println!("seq: {:?}", seq);
        println!("seq: {}", String::from_utf8(seq).unwrap());
        println!("best decode: {}", String::from_utf8(result).unwrap());
    }
}

fn hamming_distance(v1: &[u8], v2: &[u8]) -> f64 {
    let mut d: f64 = 0.0;
    for i in 0..v1.len() {
        d += ham_d(v1[i], v2[i])
    }
    d
}

fn ham_d(a: u8, b: u8) -> f64 {
    let mut mask: u8 = 128;
    let mut score: f64 = 0.0;

    while mask > 0 {
        if mask & a != mask & b { score += 1.0; }
        mask = mask >> 1;
    }

    score
}


fn prob5() {
    let seq_str: String = "ICE".to_string();
    let seq_bytes = seq_str.as_bytes();

    println!("{:?}", seq_bytes);
    
    let to_encode_str: String = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".to_string();

    let to_encode_bytes = to_encode_str.as_bytes();

    let mut encoded_bytes: Vec<u8> = vec!(0; to_encode_bytes.len());
    repeating_seq_xor(to_encode_bytes, &mut encoded_bytes, &seq_bytes);
    println!("{}", encoded_bytes[..].to_hex());
}


fn prob4() {
    let mut max_score = 0;
    let mut best_line: String = String::new();
    let mut best_decode: Vec<u8> = Vec::new();
    
    match File::open("/home/steve/rust/cryptopals/src/p4.txt") {
        Ok(t) => {
            let reader = BufReader::new(t);

            for line in reader.lines() {
                match line {
                    Ok(line_str) => {
                        let (decoded_line, _, score) =
                            decode_repeating_char_xor(&line_str.from_hex().unwrap());
                        if score > max_score {
                            max_score = score;
                            best_line = line_str;
                            best_decode = decoded_line;
                        }
                    },
                    Err(e) => println!("{:?}", e),
                }
            }
        }
        Err(e) => println!("{:?}", e),
    }
    println!("Best line: {}", best_line);
    println!("Score: {}", max_score);
    println!("Best decode: {}", String::from_utf8(best_decode).unwrap());
}

fn prob3() {
    let encoded_str: String = "".to_string();
    let encoded_bytes: Vec<u8> =
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
        .from_hex().unwrap();
    let (decoded_bytes, _, score) = decode_repeating_char_xor(&encoded_bytes);
    
    println!("{} ", String::from_utf8(decoded_bytes).unwrap());
}

fn decode_repeating_char_xor(encoded_bytes: &Vec<u8>) -> (Vec<u8>, u8, i32) {
    if DEBUG { println!("encoded_block: {:?}", encoded_bytes); }

    let mut xored_bytes: Vec<u8> = vec!(0; encoded_bytes.len());
    let mut best_decode: Vec<u8> = vec!(0; encoded_bytes.len());
    let mut letter_freqs: Vec<u32> = vec![0; 27];
    let mut max_score: i32 = std::i32::MIN;
    let mut best_key: u8 = 0;
    let mut score: i32 = 0;

    
    let char_set_string: String =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopkrstuvwxyz0123456789 \'\".,:".to_string();
    let char_set_bytes = char_set_string.as_bytes();
    
    for c in char_set_bytes {
        repeating_char_xor(&encoded_bytes, &mut xored_bytes, &c);
        calc_freqs(&xored_bytes, &mut letter_freqs);
        score = score_freqs(&letter_freqs);
        if DEBUG { println!("score for {}: {}", *c as char, score); }
        if score > max_score {
            max_score = score;
            set_vec(&xored_bytes, &mut best_decode);
            best_key = *c;
        }
        
    }
    (best_decode, best_key, max_score)
}

fn set_vec(src: &Vec<u8>, dest: &mut Vec<u8>) {
    for i in 0 .. src.len() {
        dest[i] = src[i];
    }
}

fn score_freqs(freqs: &Vec<u32>) -> i32 {
    let mut prev_max: u32 = std::u32::MAX;
    let mut cur_max: u32 = 0;

    let mut prev_let: u32 = std::u32::MAX;
    let mut cur_let: u32 = 0;
    
    let mut score: i32 = 0;
    
    for i in 0 .. 5 {
        for j in 0 .. 26 {
            if freqs[j] > cur_max && freqs[j] <= prev_max && j as u32 != prev_let  {
                cur_max = freqs[j];
                cur_let = j as u32;
            }
        }
        match cur_let {
            4 => score += 12,
            19 => score += 9,
            0 => score += 8,
            14 => score += 7,
            8 => score += 7,
            13 => score += 6,
            18 => score += 6,
            7 => score += 6,
            17 => score += 6,
            3 => score += 4,
            11 => score += 4,
            _ => (),
        }
        prev_max = cur_max;
        cur_max = 0;
        prev_let = cur_let;
        cur_let = std::u32::MAX;
    }
    score - (freqs[26] as i32 * 5)
}

fn calc_freqs(chars: &Vec<u8>, letter_freqs: &mut Vec<u32>) {

    for i in 0 .. letter_freqs.len() {
        letter_freqs[i] = 0;
    }
        
    for c in chars {
        match *c {
            b'a'...b'z' => letter_freqs[(c - b'a') as usize] += 1,
            b'A'...b'Z' => letter_freqs[(c - b'A') as usize] += 1,
            b' '| b'\'' | b'.' | b',' => (),
            _ => letter_freqs[26] += 1,
        }
    }
}

fn repeating_seq_xor(to_encode: &[u8], result: &mut Vec<u8>, seq: &[u8]) {
    for i in 0 .. to_encode.len() {
        result[i] = seq[i % seq.len()] ^ to_encode[i];
    }
}

fn repeating_char_xor(v: &Vec<u8>, result: &mut Vec<u8>, c: &u8) {
    for i in 0 .. v.len() {
        result[i] = c ^ v[i];
    }
}

fn prob2() {
    let hex1 = "1c0111001f010100061a024b53535009181c".to_string();
    let hex2 = "686974207468652062756c6c277320657965".to_string();

    let new_hex: String = to_base(&xor_vecs(&from_hex_string(&hex1)[..],
                                            &from_hex_string(&hex2)[..]), &HEX_MAP);

    println!("{:?}", new_hex);
}

fn set_xor_slice(v1: &[u8], v2: &[u8], into: &mut[u8]) {
    let l = std::cmp::min(v1.len(), v2.len());

    for i in 0..l {
        into[i] = v1[i] ^ v2[i];
    }
}

fn xor_vecs(v1: &[u8], v2: &[u8]) -> Vec<u8> {
    let mut v3: Vec<u8> = Vec::new();
    let l = std::cmp::min(v1.len(), v2.len());

    for i in 0..l {
        v3.push(v1[i] ^ v2[i]);
    }
    
    v3
}

fn prob1() {
    //let mut hex = "9ced0de1".to_string();


    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string();

    let b64: String = to_base(&from_hex_string(&hex), &B64_MAP);

    println!("hex: {}", hex);
    println!("base64: {}", b64);
}

fn test() {

    let mut hex = "FFFFFFFFFFFFFFFFFF".to_string();
    //let mut hex = "4927".to_string();
    println!("hex original: {}", hex);
    println!("binary from hex: {:?}", &from_hex_string(&hex));
    
    let b: String = to_base(&from_hex_string(&hex), &BIN_MAP);
    let h: String = to_base(&from_hex_string(&hex), &HEX_MAP);
    let b64: String = to_base(&from_hex_string(&hex), &B64_MAP);
    
    println!("base 2: {}", b);
    println!("base 16: {}", h);
    println!("base 64: {}", b64);
}


fn from_hex_string(hex: &String) -> Vec<u8> {
    let bytes = hex.as_bytes();

    let mut bin: Vec<u8> = Vec::new();
    let mut modulus: i32 = if hex.len() % 2 == 0 { 0 } else { 1 };
    let mut buf: u8 = 0;
    
    for byte in bytes {
        match *byte {
            b'0'...b'9' => buf |= byte - b'0',
            b'A'...b'F' => buf |= byte - b'A' + 10,
            b'a'...b'f' => buf |= byte - b'a' + 10,
            _ => buf |= 0,
        }
        
        if modulus == 0 {
            buf <<= 4;
            modulus = 1;
        } else {
            bin.push(buf);
            buf = 0;
            modulus = 0;
        }
    }
    
    bin
}

fn from_b64_string(b64_str: &String) -> Vec<u8> {
    let bytes = b64_str.as_bytes();
    let mut bin: Vec<u8> = Vec::new();

    let bpc: u8 = 6;
    let mut buf: u8 = 0;
    let mut working_byte: u8 = 0;
    let mut free_bits: u8 = (bytes.len() * 6 % 8) as u8;
    let mut first: bool = true;

    if free_bits == 0 { free_bits = 8 };
    
    for byte in bytes {
        match *byte {
            b'A'...b'Z' => buf = byte - b'A',
            b'a'...b'z' => buf = byte - b'a' + 26,
            b'0'...b'9' => buf = byte - b'0' + 52,
            b'+' => buf = 62,
            b'/' => buf = 63,
            _ => buf = 0,
        }

        if free_bits <= bpc {
            working_byte |= buf >> (bpc - free_bits);
            if !first || (first && working_byte != 0) {
                bin.push(working_byte);
                first = false;
            }
            working_byte = (255 ^ (255 << (bpc - free_bits))) & buf;
            if free_bits != bpc { working_byte <<= 8 - bpc + free_bits; }
            free_bits = 8 - bpc + free_bits;
        }
        else {
            working_byte |= buf << (free_bits - bpc);
            free_bits = free_bits - bpc;
        }
    }
    bin
}

fn to_base(bin: &Vec<u8>, char_map: &[char]) -> String {

    let base: u8 = char_map.len() as u8;
    let bpc: u8 = mylog2(base); // bits per character
    let mut vb64: String = String::new();
    let mut buf: u8 = 0;
    let mut leftover: u8 = 0;
    let mut offset: u8 = (bpc - (((bin.len() as u32) * 8 % (bpc as u32)) as u8))
        % bpc;
    let mut byte_mask: u8 = 255 << offset + (8 - bpc);
    let mut first: bool = true;

    for byte in bin {
        first = true;
        while first || offset >= bpc {
            if offset < bpc {
                byte_mask = 255 << offset + (8 - bpc);
                buf = leftover << (bpc - offset);
                buf |= (byte_mask & byte) >> (offset + (8 - bpc));
                offset += 8 - bpc;
                leftover = (255 ^ byte_mask) & byte;
            }
            else {
                buf = leftover >> (offset - bpc);
                offset -= bpc;
                if offset > 0 { leftover = (255 >> (8 - offset)) & leftover; }
                else { leftover = 0 };
            }
            vb64.push(char_map[buf as usize]);

            first = false;
        }
    }
    vb64
}

fn mylog2(n: u8) -> u8 {
    
    let mut x: u8 = 2;
    let mut c = 0;

    while x <= n {
        x *= 2;
        c += 1;
    }
    c
}
