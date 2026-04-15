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
