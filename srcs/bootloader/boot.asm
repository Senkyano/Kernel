bits 32

MB_ALIGN    equ 1 << 0
MB_MEMINFO  equ 1 << 1
MB_FLAGS    equ MB_ALIGN | MB_MEMINFO
MB_MAGIC    equ 0x1BADB002
MB_CHECKSUM equ -(MB_MAGIC + MB_FLAGS)

section .multiboot alloc exec write progbits
align 4
    dd MB_MAGIC
    dd MB_FLAGS
    dd MB_CHECKSUM

section .text
global _start
extern kernel_main

_start:
    mov     esp, stack_space
    push    0
    popf
    
    call    kernel_main
    
    cli
.hang:
    hlt
    jmp     .hang

section .bss
align 16
stack_bottom:
    resb 16384 ; 16 KiB de pile
stack_space: