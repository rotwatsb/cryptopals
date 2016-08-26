use std::fs::File;

use std::io::{BufRead, BufReader};

use utils::{byte_repeats_exist};

pub fn prob8() {
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

