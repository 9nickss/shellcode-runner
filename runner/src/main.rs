use std::env;
use std::fs;
use libc;

fn read_shellcode(path: &str) -> Vec<u8> {
    let data: Vec<u8> = fs::read(path).expect("Failed to read file");
    return data;
}

fn alloc_executable_memory(size: usize) -> *mut libc::c_void {
    unsafe {
        let mem = libc::mmap(std::ptr::null_mut(), size, libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
        libc::MAP_PRIVATE | libc::MAP_ANON, -1, 0);
        if mem == libc::MAP_FAILED {
            eprintln!("mmap failed");
            std::process::exit(1);
        }
        return mem;
    };
}

fn copy_to_mem(shellcode: &[u8], mem: *mut libc::c_void) {
    unsafe {
        std::ptr::copy_nonoverlapping(shellcode.as_ptr(), mem as *mut u8, shellcode.len());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Too few arguments");
        std::process::exit(1);
    }
    let shellcode = read_shellcode(&args[1]);
    let size = shellcode.len();
    let mem = alloc_executable_memory(size);
    copy_to_mem(&shellcode, mem);
    
}
