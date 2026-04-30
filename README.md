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
- **Modular Architecture** : Separate binaries for runner and encryptor with shared lib

### 🚧 In Progress / Planned
- **Polymorphic Code Generation** : Mutate shellcode at each execution
- **Junk Code Injection** : Obfuscate code patterns with meaningless instructions
- **Fileless Execution** : Execute from `/proc/self/mem` or stack instead of mmap
- **Process Injection** : Inject via ptrace or self-fork techniques
- **LD_PRELOAD Hijacking** : Load malicious shared libraries before system libraries
- **PATH Manipulation** : Replace legitimate binaries with trojaned versions
- **Cron Job Injection** : Automated persistence through scheduler
- **Living off the Land** : Leverage existing system tools for execution
- **Coordinator** : Pipeline to chain all operations together

---

## 📁 Project Structure

```
shellcode-runner/
├── Cargo.toml                 # Workspace root
├── lib/                       # Shared library
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs           # Module exports
│       ├── crypt.rs         # XOR encryption & parsing
│       └── config.rs        # Configuration struct
├── runner/                    # Shellcode executor binary
│   ├── Cargo.toml
│   └── src/
│       └── runner.rs        # Main runner with decryption support
├── encryptor/                 # XOR encryption binary
│   ├── Cargo.toml
│   └── src/
│       └── encryptor.rs     # Encrypt shellcode with custom keys
└── README.md
```

### Workspace Architecture
- **lib/** : Shared modules (crypt, config, and future polymorphizer, setup logic)
- **runner/** : Main shellcode executor with decryption
- **encryptor/** : Standalone encryption tool

---

## 🚀 Quick Start

### Compile Shellcodes

```bash
cd shellcodes
nasm -f elf64 write.asm -o write.o
objcopy -O binary write.o write.bin
```

### Build Project

```bash
cargo build --release
```

### Execute Shellcode

```bash
# Direct execution (unencrypted)
./target/release/runner -v shellcodes/write.bin

# Encrypt with XOR (default key 0xAA)
./target/release/encryptor -v shellcodes/write.bin
./target/release/runner -v shellcodes/write.bin.xor

# Encrypt with AES-128-GCM (random key generated)
./target/release/encryptor -v -a aes shellcodes/write.bin
./target/release/runner -v shellcodes/write.bin.aes

# Encrypt with custom XOR key
./target/release/encryptor -v -k FF shellcodes/write.bin
./target/release/runner -v shellcodes/write.bin.xor

# Encrypt with custom AES key
./target/release/encryptor -v -a aes -k 0123456789ABCDEF0123456789ABCDEF shellcodes/write.bin
./target/release/runner -v shellcodes/write.bin.aes
```

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

---

## 🔧 Building

```bash
# Compile entire workspace (lib + all binaries)
cargo build --release

# Or compile specific binaries
cargo build --release -p runner
cargo build --release -p encryptor
```

**Output binaries:**
```bash
./target/release/runner      # Shellcode executor
./target/release/encryptor   # XOR encryption tool
./target/release/injector    # Process injector (WIP)
```

**Dependencies:**
- `nasm` - Assembler for x86-64 shellcode
- `binutils` - For objcopy utility
- Rust 1.75+

**Cargo dependencies:**
- `libc` - C library bindings
- `clap` - Command-line argument parsing with derive macros
- `nix` - ptrace support (WIP)

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
