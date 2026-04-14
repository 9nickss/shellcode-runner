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

fn check_file(file: &str) -> Vec<u8> {
    fs::read(file).expect("Failed to read file")
}

fn save_xor_code(filename: &str, code: &mut Vec<u8>) {
    let crypted_file= format!("{filename}.xor");
    fs::write(crypted_file, code)
        .expect("Failed to write crypted shellcode");
}

fn main() {
    let args = Args::parse();
    let key: u8 = args.xor.unwrap_or(0xAA);
    let mut code: Vec<u8> = check_file(&args.file);
    crypt::xor_crypt(&mut code, key);
    save_xor_code(&args.file, &mut code);
}
