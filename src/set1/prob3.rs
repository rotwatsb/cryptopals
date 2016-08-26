use rustc_serialize::hex::{FromHex};

use std::i32;
use std::u32;

use constants::DEBUG;

pub fn prob3() {
    let encoded_bytes: Vec<u8> =
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
        .from_hex().unwrap();
    let (decoded_bytes, _, score) = decode_repeating_char_xor(&encoded_bytes);
    
    println!("{} ", String::from_utf8(decoded_bytes).unwrap());
}

pub fn decode_repeating_char_xor(encoded_bytes: &Vec<u8>) -> (Vec<u8>, u8, i32) {
    if DEBUG { println!("encoded_block: {:?}", encoded_bytes); }

    let mut xored_bytes: Vec<u8> = vec!(0; encoded_bytes.len());
    let mut best_decode: Vec<u8> = vec!(0; encoded_bytes.len());
    let mut letter_freqs: Vec<u32> = vec![0; 27];
    let mut max_score: i32 = i32::MIN;
    let mut best_key: u8 = 0;
    let mut score: i32 = 0;

    
    let char_set_string: String =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopkrstuvwxyz0123456789 \'\".,:"
        .to_string();
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

pub fn set_vec(src: &Vec<u8>, dest: &mut Vec<u8>) {
    for i in 0 .. src.len() {
        dest[i] = src[i];
    }
}

pub fn score_freqs(freqs: &Vec<u32>) -> i32 {
    let mut prev_max: u32 = u32::MAX;
    let mut cur_max: u32 = 0;

    let mut prev_let: u32 = u32::MAX;
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
        cur_let = u32::MAX;
    }
    score - (freqs[26] as i32 * 5)
}

pub fn calc_freqs(chars: &Vec<u8>, letter_freqs: &mut Vec<u32>) {

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

pub fn repeating_char_xor(v: &Vec<u8>, result: &mut Vec<u8>, c: &u8) {
    for i in 0 .. v.len() {
        result[i] = c ^ v[i];
    }
}

