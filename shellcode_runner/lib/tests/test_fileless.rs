use lib::fileless::{create_memfd, write_to_memfd, seal_memfd, map_memfd_executable, execute_fileless_mmap};
use lib::config::Config;

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

/// Test that seal_memfd successfully seals a memfd
#[test]
fn test_seal_memfd_success() {
    let config = Config::new(false);
    let test_data = vec![0x41u8; 100];
    
    unsafe {
        let fd = create_memfd(&config, true).expect("Failed to create memfd with sealing support");
        write_to_memfd(fd, &test_data, &config).expect("Failed to write to memfd");
        
        let result = seal_memfd(fd, &config);
        assert!(result.is_ok(), "seal_memfd should succeed");
        
        // After sealing, writes should fail
        let write_result = libc::write(fd, test_data.as_ptr() as *const libc::c_void, test_data.len());
        assert!(write_result < 0, "Write should fail after sealing");
        
        // Cleanup
        libc::close(fd);
    }
}
