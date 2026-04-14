# 🦀 shellcode-runner

![Rust](https://img.shields.io/badge/Rust-1.75+-orange?style=flat-square&logo=rust)
![Platform](https://img.shields.io/badge/Platform-Linux%20x86__64-blue?style=flat-square)
![Educational](https://img.shields.io/badge/Purpose-Educational%20Only-red?style=flat-square)

> ⚠️ Strictly for educational purposes. Test only on machines you own in an isolated environment.

## 🎯 Objectives

A comprehensive shellcode execution and evasion framework demonstrating advanced security concepts:

### Core Features
- **Dynamic Shellcode Execution** : Load and execute arbitrary x86-64 shellcode from binary files
- **XOR Encryption** : Encrypt shellcode with configurable XOR keys
- **Polymorphic Code Generation** : Mutate shellcode at each execution while maintaining semantic equivalence (WIP)
- **Junk Code Injection** : Insert meaningless instructions to obfuscate code patterns (WIP)

### Evasion Techniques (WIP)
- **Fileless Execution** : Execute from `/proc/self/mem` or stack instead of `mmap` to avoid kernel monitoring
- **Process Injection** : Inject shellcode into legitimate processes using `ptrace` (Linux), if it fails it will try to fork itself and inject into it's own child process
- **LD_PRELOAD Hijacking** : Load malicious shared libraries before system libraries
- **PATH Manipulation** : Replace legitimate binaries with trojaned versions
- **Cron Job Injection** : Automated persistence through scheduler-based execution
- **Living off the Land** : Leverage existing system tools for payload delivery

### Encryption & Obfuscation (WIP)
- **AES-GCM-128 Encryption** : Strong encryption for payload protection
- **Automatic Setup** : Single-binary deployment of entire evasion chain

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

## 🚀 Usage

### Compiling Shellcode to .bin

```bash
cd shellcodes

# Assemble ASM to object file
nasm -f elf64 bash.asm -o bash.o

# Extract binary from object file
objcopy -O binary bash.o bash.bin

# Verify shellcode
hexdump -C bash.bin
```

### Build Everything

```bash
cargo build --release
```

### Basic Execution

```bash
# Simple execution
./target/release/runner shellcodes/bash.bin

# With verbose logging
./target/release/runner -v shellcodes/bash.bin
./target/release/runner --verbose shellcodes/bash.bin
```

### Encrypted Execution

```bash
# Encrypt with XOR key (supports AA or 0xAA format)
./target/release/encryptor -x 0xAA shellcodes/exit.bin
./target/release/encryptor -x AA shellcodes/bash.bin
./target/release/encryptor --xor FF shellcodes/write.bin

# With verbose output
./target/release/encryptor -x 0xAA shellcodes/exit.bin -v
./target/release/encryptor --xor FF shellcodes/bash.bin --verbose

# Execute encrypted shellcode (auto-decrypt with key)
./target/release/runner -d 0xAA shellcodes/exit.bin.xor
./target/release/runner --decrypt FF shellcodes/bash.bin.xor

# Combined: verbose + decrypt
./target/release/runner -v --decrypt 0xAA shellcodes/exit.bin.xor
```

**Runner flags:**
- `-v, --verbose` : Enable verbose logging (memory addresses, syscalls, etc.)
- `-d, --decrypt <KEY>` : Decrypt shellcode with XOR key (hex format: AA or 0xAA)

**Encryptor flags:**
- `-x, --xor <KEY>` : XOR key for encryption (hex format: AA or 0xAA, default: AA)
- `-v, --verbose` : Enable verbose logging

### Process Injection (WIP)

```bash
./target/release/injector /bin/ls shellcodes/bash.bin
```

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
