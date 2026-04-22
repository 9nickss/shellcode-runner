use std::fs;
use libc;
use lib::crypt;
use clap::{Parser};
use lib::config::Config;
use lib::crypt::Algo;
use lib::crypt::Key;
mod key_parser;

#[derive(Parser)]
struct Args {
    /// File to execute
    file: String,

    /// Choose algorithm used to decrypt to override filename parsing
    #[arg(short, long, value_name = "ALGORITHM")]
    algo: Option<Algo>,

    /// Choose key to decrypt to override .key file usage
    #[arg(short, long, value_name = "KEY")]
    key: Option<String>,

    /// verbose mode
    #[arg(short, long)]
    verbose: bool,
}

fn read_shellcode(path: &str, config: &Config) -> Vec<u8> {
    config.log(&format!("Reading shellcode from: {}...", path));
    fs::read(path).expect("Failed to read file")
}

fn alloc_executable_memory(size: usize, config: &Config) -> *mut libc::c_void {
    unsafe {
        config.log(&format!("Allocating {} bytes of executable memory...", size));
        let mem = libc::mmap(std::ptr::null_mut(), size, libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
        libc::MAP_PRIVATE | libc::MAP_ANON, -1, 0);
        if mem == libc::MAP_FAILED {
            config.log("mmap faile!");
            eprintln!("mmap failed");
            std::process::exit(1);
        }
        config.log(&format!("{} bytes allocated at: 0x{:x}", size, mem as usize));
        mem
    }
}

fn copy_to_mem(shellcode: &[u8], mem: *mut libc::c_void, config: &Config) -> *mut libc::c_void {
    unsafe {
        config.log(&format!("Copying {} bytes to: 0x{:x}...", shellcode.len(), mem as usize));
        std::ptr::copy_nonoverlapping(shellcode.as_ptr(), mem as *mut u8, shellcode.len());
        config.log("Shellcode copied");
        return mem;
    };
}

fn exec(ptr: *mut libc::c_void, config: &Config) {
    unsafe {
        config.log(&format!("Executing shellcode at: 0x{:x}...", ptr as usize));
        let func = std::mem::transmute::<*mut libc::c_void, fn()>(ptr);
        func();
        config.log("Shellcode executed!");
    }
}

fn free_mem(mem: *mut libc::c_void, size: usize, config: &Config) {
    unsafe {
        config.log(&format!("Freeing {} bytes at: 0x{:x}...", size, mem as usize));
        libc::munmap(mem, size);
        config.log("Memory freed");
    }
}

fn decrypt_shellcode(shellcode: &mut Vec<u8>, file: &str, algo: Option<Algo>, key: Option<String>, config: &Config) {
    let needs_decrypt = algo.is_some()
        || file.ends_with(".xor")
        || file.ends_with(".aes");

    if !needs_decrypt { return; }

    let (_, key) = key_parser::key_parser::resolve_encryption(file, algo, key, config)
        .unwrap_or_else(|e| { eprintln!("{}", e); std::process::exit(1); });

    match &key {
        Key::Xor(k) => config.log(&format!("Decrypting with XOR key 0x{:02X}...", k)),
        Key::Aes(_)  => config.log("Decrypting with AES-128-GCM..."),
    }

    crypt::create_cipher(&key).decrypt(shellcode);
}

fn run_shellcode(shellcode: &[u8], config: &Config) {
    let size = shellcode.len();
    let mem = alloc_executable_memory(size, config);
    copy_to_mem(shellcode, mem, config);
    exec(mem, config);
    free_mem(mem, size, config);
}

fn main() {
    let args = Args::parse();
    let config = Config::new(args.verbose);

    config.log("Starting shellcode runner...");

    let mut shellcode = read_shellcode(&args.file, &config);
    decrypt_shellcode(&mut shellcode, &args.file, args.algo, args.key, &config);
    run_shellcode(&shellcode, &config);

    config.log("Done!");
}
