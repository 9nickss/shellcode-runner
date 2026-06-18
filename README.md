# 🦀 shellcode-runner

![Rust](https://img.shields.io/badge/Rust-1.75+-orange?style=flat-square&logo=rust)
![Platform](https://img.shields.io/badge/Platform-Linux%20x86__64-blue?style=flat-square)
![Status](https://img.shields.io/badge/Status-Active%20Development-blue?style=flat-square)

A modular shellcode execution framework with encryption, obfuscation, and evasion techniques.

## 🎯 Project Goals

### ✅ Current Implementation
- **Dynamic Shellcode Execution** : Load and execute x86-64 shellcode from binary files
- **XOR Encryption** : Encrypt/decrypt shellcode with configurable keys
- **AES-128-GCM Encryption** : Strong authenticated encryption with random nonces
- **Automatic Key Management** : Load encryption keys from .key files automatically
- **Verbose Logging** : Detailed output for memory addresses and execution flow
- **Pipeline Tool** : Chain encryption and execution in a single command
- **Modular Architecture** : Separate binaries for runner, encryptor, and pipeline with shared lib
- **Fileless Execution (memfd + mmap)** : Load shellcode into anonymous in-memory file descriptor, mmap with rwx, execute with return capability. Slightly visible in `/proc/[pid]/maps`
- **Fileless Execution (memfd + execveat)** : Execute directly from anonymous fd via `execveat()`. True fileless—completely invisible in maps. Process replacement only (no return)

### 🚧 In Progress / Planned
- **Polymorphic Code Generation** : Mutate shellcode at each execution
- **Junk Code Injection** : Obfuscate code patterns with meaningless instructions
- **Process Injection** : Inject via ptrace or self-fork techniques
- **LD_PRELOAD Hijacking** : Load malicious shared libraries before system libraries
- **PATH Manipulation** : Replace legitimate binaries with trojaned versions
- **Cron Job Injection** : Automated persistence through scheduler
- **Living off the Land** : Leverage existing system tools for execution

---

## 📁 Project Structure

```
shellcode-runner/
├── Cargo.toml                 # Workspace root
├── Makefile                   # Build automation
├── lib/                       # Shared library
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs           # Module exports
│       ├── crypt.rs         # XOR & AES encryption
        ├── fileless.rs      # Fileless execution
│       └── config.rs        # Configuration struct
├── runner/                    # Shellcode executor binary
│   ├── Cargo.toml
│   └── src/
│       ├── runner.rs        # Main runner with decryption
│       └── key_parser.rs    # Key resolution logic
├── encryptor/                 # Encryption binary
│   ├── Cargo.toml
│   └── src/
│       └── encryptor.rs     # Encrypt shellcode (XOR/AES)
├── pipeline/                  # Combined encryption + execution
│   ├── Cargo.toml
│   └── src/
│       └── pipeline.rs      # Chain encryptor → runner
└── README.md
```

### Workspace Architecture
- **lib/** : Shared modules (crypt, config, utilities)
- **runner/** : Main shellcode executor with automatic decryption
- **encryptor/** : Encryption tool (XOR & AES-128-GCM)
- **pipeline/** : Convenience tool to encrypt and execute in one command

---

## 🚀 Quick Start

### Compile Shellcodes

```bash
cd shellcodes
nasm -f elf64 write.asm -o write.o
objcopy -O binary write.o write.bin
```

### Build Project

You can build with **Makefile** (recommended) or directly with cargo:

#### Using Makefile
```bash
cd shellcode_runner

# Build all binaries (runner, encryptor, pipeline)
make all

# Build specific binary
make runner
make encryptor
make pipeline

# Clean build artifacts
make clean

# Full clean (remove release binaries)
make fclean

# Full rebuild
make re
```

#### Using Cargo
```bash
cargo build --release
```

### Execute Shellcode

```bash
# Direct execution (unencrypted)
./target/release/runner -v shellcodes/write.bin

# Using Pipeline (encrypt + execute in one command)
./target/release/pipeline -v shellcodes/write.bin              # XOR with default key
./target/release/pipeline -v -a aes shellcodes/write.bin       # AES with random key
./target/release/pipeline -v -k FF shellcodes/write.bin        # XOR with custom key
./target/release/pipeline -v -a aes -k 0123456789ABCDEF0123456789ABCDEF shellcodes/write.bin

# Manual encryption + execution (separate steps)
./target/release/encryptor -v shellcodes/write.bin
./target/release/runner -v shellcodes/write.bin.xor

./target/release/encryptor -v -a aes shellcodes/write.bin
./target/release/runner -v shellcodes/write.bin.aes
```

### Fileless Execution

The runner supports fileless execution strategies via `memfd_create` (Linux 3.17+):

#### Strategy 1: memfd + mmap
Load shellcode into anonymous in-memory file descriptor, map it with rwx permissions, then execute.

```bash
# Direct runner usage
./target/release/runner --fileless-mmap shellcodes/write.bin
./target/release/runner -v --fileless-mmap shellcodes/write.bin.xor
./target/release/runner -v --fileless-mmap -a aes shellcodes/write.bin.aes

# Via pipeline (encrypt + fileless execution)
./target/release/pipeline --fileless-mmap shellcodes/write.bin
./target/release/pipeline -v --fileless-mmap shellcodes/write.bin
./target/release/pipeline -v -a aes --fileless-mmap shellcodes/write.bin
./target/release/pipeline -v -a aes -k 0123456789ABCDEF0123456789ABCDEF --fileless-mmap shellcodes/write.bin
```

**Characteristics:**
- ✅ Shellcode can return control to caller
- ✅ No disk I/O for execution (encrypted source read from disk only)
- ⚠️ Slightly visible in `/proc/[pid]/maps` during execution
- ✅ Works with both XOR and AES encryption

#### Strategy 2: memfd + execveat
Execute directly from anonymous fd via `execveat()`. True fileless with complete invisibility in maps.

```bash
# Direct runner usage
./target/release/runner --fileless-execveat shellcodes/write.bin
./target/release/runner -v --fileless-execveat shellcodes/write.bin.xor
./target/release/runner -v --fileless-execveat -a aes shellcodes/write.bin.aes

# Via pipeline (encrypt + fileless execution)
./target/release/pipeline --fileless-execveat shellcodes/write_elf
./target/release/pipeline -v --fileless-execveat shellcodes/write_elf
./target/release/pipeline -v -a aes --fileless-execveat shellcodes/write_elf
./target/release/pipeline -v -a aes -k 0123456789ABCDEF0123456789ABCDEF --fileless-execveat shellcodes/write_elf
```

**Characteristics:**
- ✅ True fileless—completely invisible in `/proc/[pid]/maps`
- ⚠️ Process replacement (no return possible)
- ✅ Requires Linux 3.19+
- ✅ Works with both XOR and AES encryption

**Use when:**
- Shellcode is final (spawn shell, replace process)
- Minimal detection footprint critical
- Process replacement acceptable

**Trade-off:** No return possible (process is replaced), Linux 3.19+ required

---

## 📖 Usage Guide

### Encryptor

Encrypt shellcode with XOR or AES-128-GCM algorithms.

```bash
# Default: XOR encryption with key 0xAA
./target/release/encryptor shellcodes/write.bin
# Outputs: write.bin.xor, write.bin.xor.key

# XOR with custom key
./target/release/encryptor -k FF shellcodes/write.bin
./target/release/encryptor --key 0xFF shellcodes/write.bin

# AES-128-GCM with random key (generated automatically)
./target/release/encryptor -a aes shellcodes/write.bin
# Outputs: write.bin.aes, write.bin.aes.key

# AES with custom key (32 hex chars = 16 bytes)
./target/release/encryptor --algo aes --key 0123456789ABCDEF0123456789ABCDEF shellcodes/write.bin

# Verbose mode with logging
./target/release/encryptor -v -a aes -k 0123456789ABCDEF0123456789ABCDEF shellcodes/write.bin
```

**Encryptor Flags:**
- `-a, --algo <ALGORITHM>` : Algorithm to use (`xor` or `aes`) - Default: `xor`
- `-k, --key <KEY>` : Encryption key in hex format
  - XOR: 2 hex chars (e.g., `AA` or `0xFF`)
  - AES: 32 hex chars (e.g., `0123456789ABCDEF0123456789ABCDEF`)
  - Default: `0xAA` for XOR, random for AES
- `-v, --verbose` : Enable verbose logging

**Output Files:**
- `shellcode.bin.xor` - XOR encrypted shellcode
- `shellcode.bin.aes` - AES encrypted shellcode  
- `shellcode.bin.xor.key` - Key file (loaded automatically by runner)
- `shellcode.bin.aes.key` - Key file (loaded automatically by runner)

### Runner

Execute shellcode with automatic decryption support.

```bash
# Execute raw shellcode (no decryption)
./target/release/runner shellcodes/write.bin

# Execute XOR encrypted shellcode (auto-reads write.bin.xor.key)
./target/release/runner shellcodes/write.bin.xor

# Execute AES encrypted shellcode (auto-reads write.bin.aes.key)
./target/release/runner shellcodes/write.bin.aes

# Override encryption key via CLI
./target/release/runner -k FF shellcodes/write.bin.xor
./target/release/runner --key 0123456789ABCDEF0123456789ABCDEF shellcodes/write.bin.aes

# Override algorithm
./target/release/runner -a xor shellcodes/write.bin.xor

# Verbose logging
./target/release/runner -v shellcodes/write.bin.xor
./target/release/runner --verbose -a aes shellcodes/write.bin.aes
```

**Runner Flags:**
- `-a, --algo <ALGORITHM>` : Override detected algorithm (`xor` or `aes`)
- `-k, --key <KEY>` : Override key from file (same format as encryptor)
- `-v, --verbose` : Enable verbose logging with memory addresses

### Pipeline

Encrypt shellcode and execute it in a single command.

```bash
# Default: XOR encryption with key 0xAA, then execute
./target/release/pipeline shellcodes/write.bin

# With verbose logging
./target/release/pipeline -v shellcodes/write.bin

# AES encryption with random key, then execute
./target/release/pipeline -a aes shellcodes/write.bin

# Custom XOR key
./target/release/pipeline -k FF shellcodes/write.bin

# Custom AES key
./target/release/pipeline -a aes -k 0123456789ABCDEF0123456789ABCDEF shellcodes/write.bin

# Verbose with both options
./target/release/pipeline -v -a aes -k 0123456789ABCDEF0123456789ABCDEF shellcodes/write.bin
```

**Pipeline Flags:**
- `-a, --algo <ALGORITHM>` : Encryption algorithm (`xor` or `aes`) - Default: `xor`
- `-k, --key <KEY>` : Encryption key in hex format - Default: `0xAA` for XOR, random for AES
- `-v, --verbose` : Enable verbose logging for all operations

**Behavior:**
1. Encrypts the shellcode using the specified algorithm and key
2. Automatically executes the encrypted shellcode
3. Useful for automated workflows and testing

---

## 🧪 Testing

### Unit Tests
Test the fileless execution module:

```bash
# Run all fileless tests
cargo test --lib fileless

# Run specific test
cargo test --lib fileless::tests::test_create_memfd_success

# Run with output
cargo test --lib fileless -- --nocapture

# Run all lib tests
cargo test --lib
```

**Test Coverage:**
- ✅ `test_create_memfd_success` : Verify memfd_create creates valid fd
- ✅ `test_write_to_memfd_success` : Write and verify shellcode persistence
- ✅ `test_write_to_memfd_partial_write` : Handle large payloads (1MB)
- ✅ `test_map_memfd_executable_success` : Map with rwx permissions and verify r/w
- ✅ `test_map_memfd_executable_zero_size` : Error handling for invalid sizes
- ✅ `test_memfd_isolation` : Verify multiple memfds are isolated from each other
- ⚠️ `test_execute_fileless_mmap_nop_sled` : Full integration test (marked `#[ignore]`)

### Encryption Tests
Test XOR and AES-128-GCM encryption:

```bash
# Run all crypt tests
cargo test --lib crypt

# Run specific test
cargo test --lib crypt::tests::test_xor_crypt_restores_original_data

# Run with output
cargo test --lib crypt -- --nocapture
```

**Test Coverage:**
- ✅ `test_xor_crypt_restores_original_data` : XOR encryption is symmetric (double XOR = identity)
- ✅ `test_parse_hex_returns_correct_byte` : Hex parsing with/without 0x prefix
- ✅ `test_parse_hex_returns_error_on_invalid_input` : Error handling for invalid hex strings

---

## 🔧 Building

```bash
# Compile entire workspace (lib + all binaries)
cargo build --release

# Or compile specific binaries
cargo build --release -p runner
cargo build --release -p encryptor
cargo build --release -p pipeline
```

**Output binaries:**
```bash
./target/release/runner      # Shellcode executor
./target/release/encryptor   # Encryption tool (XOR & AES)
./target/release/pipeline    # Encrypt and execute in one command
```

**Dependencies:**
- `nasm` - Assembler for x86-64 shellcode
- `binutils` - For objcopy utility
- Rust 1.75+

**Cargo dependencies:**
- `libc` - C library bindings (syscalls, mmap, etc.)
- `clap` - Command-line argument parsing with derive macros
- `aes-gcm` - AES-128-GCM encryption with authenticated encryption
- `nix` - ptrace and syscall support (for future features)

---

## 📚 Key Concepts

### Polymorphic Code
Shellcode mutates at each execution through instruction reordering, register substitution, and equivalent instruction replacement.

### Junk Code Injection
Random meaningless instructions added to confuse signature analysis.

### Fileless Execution
Execute directly from `/proc/self/mem` or stack memory instead of mmapped regions, avoiding kernel-level AV hooks.

### Process Injection
Attach to running processes via `ptrace`, modify execution context, and inject shellcode invisibly.

### LD_PRELOAD Hijacking
Load malicious shared libraries before system libraries, with constructor functions executing before `main()`.

### Living off the Land
Use existing system tools for payload delivery without writing custom binaries.

---

## ⚠️ Legal & Ethical Notice

**This code is for educational purposes only. Test only on systems you own.**
