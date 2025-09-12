.intel_syntax noprefix

.global _start

_start:
    xor ebp, ebp
    mov rdi, rsp
    call _main

.section .note.GNU-stack,"",@progbits
