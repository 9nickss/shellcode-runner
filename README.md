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

### 🚧 In Progress / Planned
- **Fileless Execution (In Progress)** : Multiple strategies for executing shellcode without disk traces:
  - **Approach 1: memfd + mmap** : Load shellcode into anonymous file descriptor, mmap it with rwx permissions, then execute. Shellcode can return control to caller. Slightly visible in `/proc/[pid]/maps` during execution
  - **Approach 2: memfd + execveat** : Load shellcode into anonymous file descriptor, execute directly via `execveat()` with `AT_EMPTY_PATH` flag. True fileless—completely invisible in maps. Process replacement (cannot return)
  - **Runner supports both modes** : Use the appropriate execution method based on shellcode requirements (return vs. process replacement)
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

The runner supports two fileless execution strategies via `memfd_create` (Linux 3.17+):

#### Strategy 1: memfd + mmap (flexible)
```bash
# Shellcode loaded into anonymous in-memory file descriptor
# Mapped into executable memory with mmap, then executed
./target/release/runner --fileless-mmap shellcodes/write.bin
./target/release/runner -v --fileless-mmap shellcodes/write.bin.xor
```
**Use when:**
- Shellcode needs to return control to caller
- Multiple operations required from same binary
- Fine-grained memory control needed

**Trade-off:** Slightly visible in `/proc/[pid]/maps` during execution

#### Strategy 2: memfd + execveat (true fileless)
```bash
# Shellcode loaded into anonymous in-memory file descriptor
# Executed directly with execveat, replacing the current process
./target/release/runner --fileless-execveat shellcodes/write.bin
./target/release/runner -v --fileless-execveat shellcodes/write.bin.xor
```
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
