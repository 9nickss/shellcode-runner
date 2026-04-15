use clap;

#[derive(clap::ValueEnum, Clone)]
pub enum Algo {
    Xor,
    Aes,
}

pub fn xor_crypt(code: &mut Vec<u8>, key: u8){
        for byte in code.iter_mut() {
            *byte ^= key;
        }
    }

pub fn parse_hex(s: &str) -> Result<u8, String> {
    let cleaned = s.trim_start_matches("0x").trim_start_matches("0X");
    u8::from_str_radix(cleaned, 16)
        .map_err(|_| format!("Invalid hex value: {}", s))
}

pub trait Cipher {
    fn encrypt(&self, data: &mut Vec<u8>);
    fn decrypt(&self, data: &mut Vec<u8>);
}

pub struct Xor { key: u8 }
pub struct Aes { key: [u8; 16] }

impl Cipher for Xor {
    fn encrypt(&self, data: &mut Vec<u8>) { 
        xor_crypt(data, self.key);
    }
    fn decrypt(&self, data: &mut Vec<u8>) {
        xor_crypt(data, self.key);
    }
}

impl Cipher for Aes {
    fn encrypt(&self, data: &mut Vec<u8>) { /* aes */ }
    fn decrypt(&self, data: &mut Vec<u8>) { /* aes */ }
}
pub fn create_cipher(algo: &Algo, key: u8) -> Box<dyn Cipher> {
    match algo {
        Algo::Xor => Box::new(Xor { key } ),
        Algo::Aes => Box::new(Aes { key: [key; 16] } ),
    }
}
