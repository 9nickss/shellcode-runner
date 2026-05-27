use lib::crypt::{create_cipher, Key};

#[test]
fn test_xor_cipher() {
    let original = vec![0x41, 0x42, 0x43];
    let mut data = original.clone();
    let cipher = create_cipher(&Key::Xor(0xAA));

    cipher.encrypt(&mut data);
    cipher.decrypt(&mut data);

    assert_eq!(data, original);
}

#[test]
fn test_aes_cipher() {
    let original = vec![0x41, 0x42, 0x43];
    let mut data = original.clone();
    let key = [0u8; 16];
    let cipher = create_cipher(&Key::Aes(key));

    cipher.encrypt(&mut data);
    cipher.decrypt(&mut data);

    assert_eq!(data, original);
}