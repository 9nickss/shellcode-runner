; exit.asm
section .text
global _start
_start:
    xor rdi, rdi     ; rdi = 0  (exit code)
    mov rax, 60      ; syscall exit = 60
    syscall