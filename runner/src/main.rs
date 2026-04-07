use std::env;
use std::fs;
use libc;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Too few arguments");
        std::process::exit(1);
    }
    let data: Vec<u8> = fs::read(&args[1]).expect("Failed to read file");
    let mem = unsafe {
        libc::mmap(std::ptr::null_mut(), data.len(), libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
        libc::MAP_PRIVATE | libc::MAP_ANON, -1, 0)
    };
}
