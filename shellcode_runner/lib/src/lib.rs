pub mod config;
pub mod crypt {
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
}
