use rustc_serialize::hex::{FromHex};

use std::io::BufReader;
use std::io::BufRead;

use std::fs::File;

use set1::prob3::decode_repeating_char_xor;

pub fn prob4() {
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
