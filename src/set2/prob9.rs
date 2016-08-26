use utils::{pad_to};

pub fn prob9() {
    let mut x: Vec<u8> = "YELLOW SUBMARINE".to_string().as_bytes().to_vec();
    pad_to(&mut x, 15);
    println!("{:?}", String::from_utf8(x).unwrap());
}
