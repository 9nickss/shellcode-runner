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
├── runner/
│   ├── src/
│   │   ├── main.rs           # Shellcode loader & executor
│   │   ├── encryptor.rs      # XOR encryption & key handling
│   │   ├── polymorphizer.rs  # Polymorphic code generation
│   │   ├── injector.rs       # Process injection (ptrace)
│   │   └── setup.rs          # Automated evasion setup
│   ├── shellcodes/
│   │   ├── bash.asm          # /bin/bash spawner
│   │   ├── exit.asm          # Process exit
│   │   └── write.asm         # Write to stdout
│   └── Cargo.toml
└── README.md
```

---

## 🚀 Usage

### Compiling Shellcode to .bin

```bash
cd runner/shellcodes

# Assemble ASM to object file
nasm -f elf64 bash.asm -o bash.o

# Extract binary from object file
objcopy -O binary bash.o bash.bin

# Verify shellcode
hexdump -C bash.bin
```

### Basic Execution

```bash
cd runner
cargo build --release
./target/release/runner shellcodes/bash.bin
```

### Encrypted Execution

```bash
./target/release/encryptor shellcodes/bash.bin 0xAA
./target/release/runner shellcodes/bash.bin.encrypted
```

### Process Injection

```bash
./target/release/injector /bin/ls shellcodes/bash.bin
```

### Automated Evasion Setup

```bash
./target/release/setup
```

---

## 🔧 Building

```bash
cd runner
cargo build --release
```

**Dependencies:**
- `nasm` - Assembler for x86-64 shellcode
- `nix` - for ptrace
- Rust 1.75+

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
