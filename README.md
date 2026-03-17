# 🦀 shellcode-runner

![Rust](https://img.shields.io/badge/Rust-1.75+-orange?style=flat-square&logo=rust)
![Platform](https://img.shields.io/badge/Platform-Linux%20x86__64-blue?style=flat-square)
![Educational](https://img.shields.io/badge/Purpose-Educational%20Only-red?style=flat-square)

> ⚠️ Strictly for educational purposes. Test only on machines you own in an isolated environment.

A shellcode runner written in Rust. Write your own x86_64 shellcode in assembly, assemble it with `nasm`, and the runner will load it into executable memory and run it. Built to understand low-level memory execution and OS memory permissions.

---

## Usage

**Requirements:** Rust 1.75+, nasm, Linux x86_64

```bash
git clone https://github.com/9nickss/shellcode-runner
cd shellcode-runner

# Write your shellcode in shellcodes/your_shellcode.asm, then assemble it
nasm -f elf64 shellcodes/your_shellcode.asm -o shellcodes/your_shellcode.o
objcopy -O binary shellcodes/your_shellcode.o shellcodes/your_shellcode.bin

# Build and run
cargo build --release
./target/release/shellcode-runner shellcodes/your_shellcode.bin
```

