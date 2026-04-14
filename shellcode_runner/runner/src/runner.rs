use std::fs;
use libc;
use lib::crypt;
use clap::{Parser};
use lib::config::Config;

#[derive(Parser)]
struct Args {
    /// File to execute
    file: String,

    /// Choose key to decrypt
    #[arg(short, long, value_name = "KEY")]
    decrypt: Option<String>,
}

fn read_shellcode(path: &str) -> Vec<u8> {
    fs::read(path).expect("Failed to read file")
}

fn alloc_executable_memory(size: usize) -> *mut libc::c_void {
    unsafe {
        let mem = libc::mmap(std::ptr::null_mut(), size, libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
        libc::MAP_PRIVATE | libc::MAP_ANON, -1, 0);
        if mem == libc::MAP_FAILED {
            eprintln!("mmap failed");
            std::process::exit(1);
        }
        mem
    }
}

fn copy_to_mem(shellcode: &[u8], mem: *mut libc::c_void) -> *mut libc::c_void {
    unsafe {
        std::ptr::copy_nonoverlapping(shellcode.as_ptr(), mem as *mut u8, shellcode.len());
        return mem;
    };
}

fn exec(ptr: *mut libc::c_void) {
    unsafe {
        let func = std::mem::transmute::<*mut libc::c_void, fn()>(ptr);
        func();
    }
}

fn free_mem(mem: *mut libc::c_void, size: usize) {
    unsafe {
        libc::munmap(mem, size);
    }
}

fn main() {
    let args = Args::parse();
    let mut shellcode: Vec<u8> = read_shellcode(&args.file);
    if let Some(key_str) = args.decrypt {
        crypt::xor_crypt(&mut shellcode,
            crypt::parse_hex(&key_str).expect("Bad key"));
    }
    let size: usize = shellcode.len();
    let mem: *mut libc::c_void = alloc_executable_memory(size);
    exec(copy_to_mem(&shellcode, mem));
    free_mem(mem, size);
}
