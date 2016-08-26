use rand::{OsRng, Rng};

pub fn prob12() {
    let mut g: OsRng = OsRng::new().unwrap();
    let mut key: Vec<u8> = g.gen_iter().take(16).collect::<Vec<u8>>();

    
}
