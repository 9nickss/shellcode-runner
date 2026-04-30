section .text
global _start
_start:
    xor rax, rax
    push rax
    mov rax, 0x6f6c6c6568
    push rax
    mov rax, 1
    mov rdi, 1
    mov rsi, rsp
    mov rdx, 5
    syscall
    pop rax     ; dépile "hello"
    pop rax     ; dépile le null
    ret