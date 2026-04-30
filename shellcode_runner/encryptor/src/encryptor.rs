use std::fs;
use lib::crypt::{create_cipher};
use clap::{Parser};
use lib::config::Config;
use lib::crypt::Algo;
use lib::crypt::Key;
use aes_gcm::{
    aead::OsRng,
    aead::rand_core::RngCore,
};
use lib::crypt;
#[derive(Parser)]
struct Args {
    /// Binary to crypt
    file: String,

    /// Choose algorithm used to crypt, by default: xor
    #[arg(short, long, value_name = "ALGORITHM")]
    algo: Option<Algo>,

    /// Choose key used to crypt
    #[arg(short, long, value_name = "KEY")]
    key: Option<String>,

    ///verbose mode
    #[arg(short, long)]
    verbose: bool,
}

fn check_file(file: &str, config: &Config) -> Vec<u8> {
    config.log(&format!("Reading shellcode from: {}...", file));
    fs::read(file).expect("Failed to read file")
}

fn save_crypted_file(filename: &str, code: &mut Vec<u8>, config: &Config, algo: &Algo, key: &Key) {
    let crypted_file = match algo {
        Algo::Xor => format!("{filename}.xor"),
        Algo::Aes => format!("{filename}.aes"),
    };
    config.log(&format!("Writing {} bytes to {}...", code.len(), crypted_file));
    fs::write(&crypted_file, code)
        .expect("Failed to write crypted shellcode");
    config.log(&format!("Code written to {}", &crypted_file));
    save_key_file(&crypted_file, &key, &config);
}

fn save_key_file(filename: &str, key: &Key, config: &Config) {
    config.log(&format!("Saving key to file {}.key...", filename));
    match &key {
        Key::Xor(k) => fs::write(format!("{}.key", filename), format!("{:02X}", k))
            .expect("Failed to write key file"),
        Key::Aes(k) => fs::write(format!("{}.key", filename), 
            k.iter().map(|b| format!("{:02X}", b)).collect::<String>())
            .expect("Failed to write key file"),
    }
    config.log("Key saved!");
}

fn main() {
    let args = Args::parse();
    let config = Config::new(args.verbose);
    let algo = args.algo.unwrap_or(Algo::Xor);
    let key = match &algo {
        Algo::Xor => Key::Xor(args.key
        .map(|k| lib::crypt::parse_hex(&k)
            .unwrap_or_else(|e| { eprintln!("{}", e); std::process::exit(1); }))
        .unwrap_or(0xAA)),

        Algo::Aes => match args.key {
            Some(k) => Key::Aes(crypt::parse_hex_bytes(&k)
                .unwrap_or_else(|e| { eprintln!("{}", e); std::process::exit(1); })),
            None => {
                let mut key_bytes = [0u8; 16];
                OsRng.fill_bytes(&mut key_bytes);
                Key::Aes(key_bytes)
            }
        },
    };

    match &key {
        Key::Xor(k) => config.log(&format!("Encrypting with XOR key 0x{:02X}...", k)),
        Key::Aes(_) => config.log("Encrypting with AES-128-GCM...",)
    };

    let cipher = create_cipher(&key);
    let mut code: Vec<u8> = check_file(&args.file, &config);
    cipher.encrypt(&mut code);
    config.log("Code encrypted!");
    save_crypted_file(&args.file, &mut code, &config, &algo, &key);
    config.log("Done!");
}
