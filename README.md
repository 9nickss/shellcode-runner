# 🦀 shellcode-runner

![Rust](https://img.shields.io/badge/Rust-1.75+-orange?style=flat-square&logo=rust)
![Platform](https://img.shields.io/badge/Platform-Linux%20x86__64-blue?style=flat-square)
![Status](https://img.shields.io/badge/Status-Active%20Development-blue?style=flat-square)

A modular shellcode execution framework with encryption, obfuscation, and evasion techniques.

## 🎯 Project Goals

### ✅ Current Implementation
- **Dynamic Shellcode Execution** : Load and execute x86-64 shellcode from binary files
- **XOR Encryption** : Encrypt/decrypt shellcode with configurable keys
- **Verbose Logging** : Detailed output for memory addresses and execution flow
- **Modular Architecture** : Separate binaries for runner, encryptor, and future tools

### 🚧 In Progress / Planned
- **AES-GCM Encryption** : Strong encryption for payload protection
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
nasm -f elf64 bash.asm -o bash.o
objcopy -O binary bash.o bash.bin
```

### Build Project

```bash
cargo build --release
```

### Execute Shellcode

```bash
# Direct execution
./target/release/runner -v shellcodes/bash.bin

# Encrypt first
./target/release/encryptor -v -x AA shellcodes/bash.bin
# Creates: bash.bin.xor and bash.bin.key

# Decrypt and execute
./target/release/runner -v shellcodes/bash.bin.xor
# Auto-reads bash.bin.key for decryption
```

---

## 📖 Usage Guide

### Runner

```bash
# Encrypt with default algorithm (XOR) and default key (0xAA)
./target/release/encryptor shellcodes/bash.bin

# Specify algorithm
./target/release/encryptor -a xor shellcodes/bash.bin
./target/release/encryptor --algo aes shellcodes/bash.bin

# Specify key (hex format: AA or 0xAA)
./target/release/encryptor -k AA shellcodes/bash.bin
./target/release/encryptor --key 0xFF shellcodes/bash.bin

# Both algorithm and key
./target/release/encryptor -a xor -k FF shellcodes/bash.bin
./target/release/encryptor --algo aes --key 0x0123456789ABCDEF shellcodes/bash.bin

# With verbose output
./target/release/encryptor -v -a xor -k AA shellcodes/bash.bin
./target/release/encryptor --verbose --algo xor --key 0xFF shellcodes/write.bin

# Execute encrypted shellcode (auto-decrypt with key)
./target/release/runner -v shellcodes/bash.bin.xor
./target/release/runner --verbose shellcodes/bash.bin.aes
```

**Encryptor flags:**
- `-a, --algo <ALGORITHM>` : Algorithm (xor, aes) - default: xor
- `-k, --key <KEY>` : Encryption key in hex (AA or 0xAA) - default: 0xAA for xor
- `-v, --verbose` : Detailed logging

**Output:**
- `shellcode.bin.xor` - XOR encrypted shellcode
- `shellcode.bin.aes` - AES encrypted shellcode
- `shellcode.bin.key` - Key file for decryption

**Runner flags:**
- `-v, --verbose` : Enable verbose logging (memory addresses, syscalls, etc.)
- `--decrypt <KEY>` : Decrypt shellcode with key (hex format: AA or 0xAA)

### Encryptor

```bash
# Default XOR encryption (key=0xAA)
./target/release/encryptor shellcodes/bash.bin

# Custom XOR key
./target/release/encryptor -x AA shellcodes/bash.bin
./target/release/encryptor --xor 0xFF shellcodes/bash.bin

# Verbose output
./target/release/encryptor -v -x AA shellcodes/bash.bin
```

**Flags:**
- `-x, --xor <KEY>` : XOR key in hex (AA or 0xAA, default: AA)
- `-v, --verbose` : Detailed logging

**Output:**
- `shellcode.bin.xor` - Encrypted shellcode
- `shellcode.bin.key` - Key file for decryption

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
