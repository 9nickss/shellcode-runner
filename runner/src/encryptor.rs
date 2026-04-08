use std::env;
use std::fs;

fn check_file(file: &str) -> Vec<u8> {
    fs::read(file).expect("Failed to read file")
}

fn parse_key(key: &str) -> u8 {
    u8::from_str_radix(key.strip_prefix("0x").expect("Invalid key prefix"), 16)
        .expect("Invalid hex value in key") // 16 car hexadecimal (XOR)
}

fn check_args_key(args: &Vec<String>) -> u8 {
    if args.len() < 2 || args.len() > 3 { // avec key custom ou key de base
        eprintln!("Wrong number of arguments");
        std::process::exit(1);
    }
    if args.len() == 3 {
        return parse_key(&args[2]);
    }
    0xAA
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let key = check_args_key(&args);
    check_file(&args[1]);
}
