.intel_syntax noprefix

.global _write
.global _read
.global _execve
.global _fork
.global _waitid
.global _exit

_write:
    mov rax, 1
    syscall
    ret

_read:
    mov rax, 0
    syscall
    ret

_execve:
    mov rax, 59
    syscall
    ret

_fork:
    mov rax, 57
    syscall
    ret

_waitid:
    mov rax, 247
    mov r10, rcx
    syscall
    ret

_exit:
    mov rax, 60
    syscall

.section .note.GNU-stack,"",@progbits