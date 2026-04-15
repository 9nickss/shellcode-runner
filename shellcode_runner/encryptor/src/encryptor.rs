use std::fs;
use lib::crypt::{create_cipher};
use clap::{Parser};
use lib::config::Config;
use lib::crypt::Algo;

#[derive(Parser)]
struct Args {
    /// Binary to crypt
    file: String,

    /// Choose algorithm used to crypt, by default: xor
    #[arg(short, long, value_name = "ALGORITHM")]
    algo: Option<Algo>,

    /// Choose key used to crypt
    #[arg(short, long, value_name = "KEY")]
    key : Option<u8>,

    ///verbose mode
    #[arg(short, long)]
    verbose: bool,
}

fn check_file(file: &str, config: &Config) -> Vec<u8> {
    config.log(&format!("Reading shellcode from: {}...", file));
    fs::read(file).expect("Failed to read file")
}

fn save_crypted_file(filename: &str, code: &mut Vec<u8>, config: &Config, algo: &Algo) {
    let crypted_file = match algo {
        Algo::Xor => format!("{filename}.xor"),
        Algo::Aes => format!("{filename}.aes"),
    };
    config.log(&format!("Writing {} bytes to {}...", code.len(), crypted_file));
    fs::write(&crypted_file, code)
        .expect("Failed to write crypted shellcode");
    config.log(&format!("Code written to {}", &crypted_file));
}

fn main() {
    let args = Args::parse();
    let config = Config::new(args.verbose, None);
    let algo = args.algo.unwrap_or(Algo::Xor);
    let key = args.key.unwrap_or_else(|| match algo {
        Algo::Xor => 0xAA,
        Algo::Aes => todo!("default aes key"),
    });
    let cipher = create_cipher(&algo, key);
    let mut code: Vec<u8> = check_file(&args.file, &config);
    config.log(&format!("Encrypting with key 0x{:02X}...", key));
    cipher.encrypt(&mut code);
    config.log("Code encrypted!");
    save_crypted_file(&args.file, &mut code, &config, &algo);
    config.log("Done!");
}
