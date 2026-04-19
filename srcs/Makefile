CC       = gcc
CFLAGS   = -m32 -ffreestanding -O2 -Wall -Wextra -fno-stack-protector
ASM      = nasm
ASMFLAGS = -f elf32

# Chemins absolus depuis la racine du container
ROOT    = /kernel
OUTDIR  = $(ROOT)/output
ISODIR  = $(ROOT)/isodir
LINKER  = $(ROOT)/linker.ld

SRCS_ASM	:= bootloader/boot.asm
SRCS_C		:= kernelspace/kernel.c
OBJS		:= boot.o kernel.o

all: $(OUTDIR)/kernel.elf iso

$(KERNEL_LIB):
	cd $(KERNEL_CRATE) && \
	cargo build --release --target i686-unknown-linux-gnu

boot.o: $(SRCS_ASM)
	$(ASM) $(ASMFLAGS) $< -o $@

$(OUTDIR)/kernel.elf: $(OBJS)
	mkdir -p $(OUTDIR)
	ld -m elf_i386 -T $(LINKER) --oformat elf32-i386 -o $@ $^

iso: $(OUTDIR)/kernel.elf
	mkdir -p $(ISODIR)/boot/grub
	cp $(OUTDIR)/kernel.elf $(ISODIR)/boot/kernel.elf
	printf 'set timeout=0\nset default=0\nmenuentry "MyKernel" {\n  multiboot /boot/kernel.elf\n}\n' \
		> $(ISODIR)/boot/grub/grub.cfg
	grub-mkrescue -o $(OUTDIR)/kernel.iso $(ISODIR)

clean:
	rm -f *.o
	rm -rf $(OUTDIR) $(ISODIR)