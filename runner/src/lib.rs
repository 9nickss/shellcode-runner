pub mod crypt {
    pub fn xor_crypt(code: &mut Vec<u8>, key: u8){
        for byte in code.iter_mut() {
            *byte ^= key;
        }
    }
}
