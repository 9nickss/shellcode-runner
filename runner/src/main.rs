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
    unsafe {
        let mem = libc::mmap(std::ptr::null_mut(), data.len(), libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
        libc::MAP_PRIVATE | libc::MAP_ANON, -1, 0);
        if mem == libc::MAP_FAILED {
            eprintln!("mmap failed");
            std::process::exit(1);
        }
        std::ptr::copy_nonoverlapping(data.as_ptr(), mem as *mut u8, data.len());
    };
}
