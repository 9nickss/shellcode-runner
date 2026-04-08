use std::env;
use std::fs;
use libc;

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
        mem
    }
}

fn exec(ptr: *mut libc::c_void) {
    unsafe {
        let func = std::mem::transmute::<*mut libc::c_void, fn()>(ptr);
        func();
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
    exec(copy_to_mem(&shellcode, mem));
}
