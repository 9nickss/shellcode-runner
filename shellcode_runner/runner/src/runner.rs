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

    /// verbose mode
    #[arg(short, long)]
    verbose: bool,
}

fn read_shellcode(path: &str, config: &Config) -> Vec<u8> {
    config.log(&format!("Reading shellcode from: {} ...", path));
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

fn main() {
    let args = Args::parse();
    let mut config = Config::new(args.verbose, None);
    config.log("Starting shellcode runner...");
    let mut shellcode: Vec<u8> = read_shellcode(&args.file, &config);
    if let Some(key_str) = args.decrypt {
        config.log(&format!("Decrypting with key: {}...", key_str));
        config.key = crypt::parse_hex(&key_str).ok();
        if let Some(key) = config.key {
            crypt::xor_crypt(&mut shellcode, key);
            config.log(&format!("Decrypted with key 0x{:02X}", key));
        } else {
            config.log("Invalid hex key");
            eprintln!("Error: Invalid hexcode");
            std::process::exit(1);
        }
    }
    let size: usize = shellcode.len();
    config.log(&format!("Shellcode size: {} bytes", size));
    let mem: *mut libc::c_void = alloc_executable_memory(size, &config);
    copy_to_mem(&shellcode, mem, &config);
    exec(mem, &config);
    free_mem(mem, size, &config);
    config.log("Done!");
}
