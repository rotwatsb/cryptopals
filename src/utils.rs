use rustc_serialize::base64::{FromBase64};
    
use crypto::buffer::{RefReadBuffer, RefWriteBuffer};
use crypto::symmetriccipher::{Decryptor, Encryptor};
use crypto::aes::{KeySize, ecb_encryptor, ecb_decryptor};
use crypto::blockmodes::{NoPadding};

use std::fs::File;
use std::io::{BufRead, BufReader};

use std::cmp;

use std::collections::BTreeMap;

pub fn ecb_decrypt(slice_in: &[u8], slice_out: &mut [u8], key: &[u8]) {
    
    let mut boxed_dec: Box<Decryptor> =
        ecb_decryptor(KeySize::KeySize128, &key[..], NoPadding);

    let mut buff_in = RefReadBuffer::new(slice_in);
    let mut buff_out = RefWriteBuffer::new(slice_out);
    
    (*boxed_dec).decrypt(&mut buff_in, &mut buff_out, true);
    
}

pub fn ecb_encrypt(slice_in: &[u8], slice_out: &mut [u8], key: &[u8]) {
    
    let mut boxed_enc: Box<Encryptor> =
        ecb_encryptor(KeySize::KeySize128, &key[..], NoPadding);

    let mut buff_in = RefReadBuffer::new(slice_in);
    let mut buff_out = RefWriteBuffer::new(slice_out);
    
    (*boxed_enc).encrypt(&mut buff_in, &mut buff_out, true);
}

pub fn pad_to(to_pad: &mut Vec<u8>, block_size: usize) {
    let to_append: usize = (block_size - (to_pad.len() % block_size)) % block_size;

    for i in 0 .. to_append {
        to_pad.push(to_append as u8);
    }
}

pub fn cbc_encrypt(bin: &mut Vec<u8>, iv: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {

    let chunksize: usize = iv.len();
    pad_to(bin, chunksize);
    
    let mut bin2: Vec<u8> = bin.clone();
    let mut vec_out: Vec<u8> = vec!(0; bin.len());
    let l: usize = bin2.len();

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

pub fn cbc_decrypt(bin: &Vec<u8>, iv: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {

    let chunksize: usize = iv.len();

    let mut vec_out: Vec<u8> = vec!(0; bin.len());
    let mut vec_out2: Vec<u8> = vec_out.clone();
    let l: usize = bin.len();

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

pub fn bin_from_b64file(f: &str) -> Vec<u8> {
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

pub fn set_xor_slice(v1: &[u8], v2: &[u8], into: &mut[u8]) {
    let l = cmp::min(v1.len(), v2.len());

    for i in 0..l {
        into[i] = v1[i] ^ v2[i];
    }
}

pub fn byte_repeats_exist(data: &Vec<u8>, n: usize) -> bool {

    let mut block_map: BTreeMap<&[u8], u32> = BTreeMap::new();

    for i in 0 .. data.len() / n {
        let x: &[u8] = &data[i*n..cmp::min(data.len(), i*n+n)];
        if !block_map.contains_key(&x) {
            block_map.insert(x, 1);
        }
        else { return true; }
    }

    false
}
