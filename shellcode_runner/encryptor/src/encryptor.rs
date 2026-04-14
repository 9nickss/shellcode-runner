use std::fs;
use lib::crypt;
use clap::{Parser};
use lib::config::Config;

#[derive(Parser)]
struct Args {
    /// Binary to crypt
    file: String,

    /// Choose XOR key to crypt, default = 0xAA
    #[arg(short, long, value_name = "KEY", value_parser = crypt::parse_hex)]
    xor: Option<u8>,

    ///verbose mode
    #[arg(short, long)]
    verbose: bool,
}

fn check_file(file: &str, config: &Config) -> Vec<u8> {
    config.log(&format!("Reading shellcode from: {}...", file));
    fs::read(file).expect("Failed to read file")
}

fn save_xor_code(filename: &str, code: &mut Vec<u8>, config: &Config) {
    let crypted_file= format!("{filename}.xor");
    config.log(&format!("Writing {} bytes to {}...", code.len(), crypted_file));
    fs::write(&crypted_file, code)
        .expect("Failed to write crypted shellcode");
    config.log(&format!("Code written to {}", &crypted_file));
}

fn main() {
    let args = Args::parse();
    let config = Config::new(args.verbose, None);
    let key: u8 = args.xor.unwrap_or(0xAA);
    let mut code: Vec<u8> = check_file(&args.file, &config);
    config.log(&format!("Encrypting with key 0x{:02X}...", key));
    crypt::xor_crypt(&mut code, key);
    config.log("Code encrypted!");
    save_xor_code(&args.file, &mut code, &config);
    config.log("Done!");
}
