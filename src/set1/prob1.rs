pub const BIN_MAP: [char; 2] = ['0', '1'];

pub const HEX_MAP: [char; 16] =
    ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd',
     'e', 'f'];

pub const B64_MAP: [char; 64] =
    ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
     'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b',
     'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
     'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3',
     '4', '5', '6', '7', '8', '9', '+', '/'];

pub fn prob1() {
    //let mut hex = "9ced0de1".to_string();


    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string();

    let b64: String = to_base(&from_hex_string(&hex), &B64_MAP);

    println!("hex: {}", hex);
    println!("base64: {}", b64);
}

pub fn from_hex_string(hex: &String) -> Vec<u8> {
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

pub fn from_b64_string(b64_str: &String) -> Vec<u8> {
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

pub fn to_base(bin: &Vec<u8>, char_map: &[char]) -> String {

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

pub fn mylog2(n: u8) -> u8 {
    
    let mut x: u8 = 2;
    let mut c = 0;

    while x <= n {
        x *= 2;
        c += 1;
    }
    c
}

fn test() {

    let hex = "FFFFFFFFFFFFFFFFFF".to_string();
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
