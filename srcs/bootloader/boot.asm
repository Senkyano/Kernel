MAGIC		equ 9x1BADB002
FLAGS		equ 0x0
CHECKSUM	equ -(MAGIC + FLAGS)

section .multiboot
dd	MAGIC
dd	FLAGS
dd	CHECKSUM

section .text
global	_start

_start:
	mov		esp, stack_top
	call	kernel_main
	htl

section	.bss
resb	16384
stack_top: