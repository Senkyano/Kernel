CC       = gcc
CFLAGS   = -m32 -U__linux__ -ffreestanding -O2 -Wall -Wextra -fno-stack-protector -nostdlib
ASM      = nasm
ASMFLAGS = -f elf32

ROOT   = /kernel
OUTDIR = $(ROOT)/output
ISODIR = $(ROOT)/isodir
LINKER = $(ROOT)/linker.ld

SRCS_ASM     = bootloader/boot.asm
SRCS_C       = kernelspace_c/kernel.c
KERNEL_CRATE = $(ROOT)/srcs/kernelspace_rust
KERNEL_TARGET = i386-kernel
KERNEL_LIB   = $(KERNEL_CRATE)/target/$(KERNEL_TARGET)/release/libkernel.a

# ── Colors ────────────────────────────────────────────────────────
C_B = \033[0;30m
C_R = \033[1;31m
C_G = \033[1;32m
C_Y = \033[0;33m
C_BU = \033[0;34m
C_M = \033[0;35m
C_C = \033[0;36m
C_W = \033[0;37m
RESET = \033[0m

# ── Sélection du mode ─────────────────────────────────────────────
# make          → C  (défaut)
# make LANG=rs  → Rust
LANG ?= c

ifeq ($(LANG),rs)
  OBJS        = boot.o
  LINK_EXTRAS = --whole-archive $(KERNEL_LIB) --no-whole-archive
  OUTDIR_LANG = $(OUTDIR)/rust
  KERNEL_ELF  = $(OUTDIR_LANG)/kernel.elf
  KERNEL_ISO  = $(OUTDIR_LANG)/kernel.iso
  ISODIR_LANG = $(ISODIR)/rust
else
  OBJS        = boot.o kernel_c.o
  LINK_EXTRAS =
  OUTDIR_LANG = $(OUTDIR)/c
  KERNEL_ELF  = $(OUTDIR_LANG)/kernel.elf
  KERNEL_ISO  = $(OUTDIR_LANG)/kernel.iso
  ISODIR_LANG = $(ISODIR)/c
endif

# ── Targets principales ───────────────────────────────────────────
.PHONY: all iso clean fclean re

all: $(KERNEL_ELF) iso

# ── Compilation Rust ──────────────────────────────────────────────
$(KERNEL_LIB):
	cd $(KERNEL_CRATE) && \
	RUST_TARGET_PATH=$(KERNEL_CRATE) \
	RUSTFLAGS="-Z unstable-options -Z ub-checks=no" \
	cargo +nightly build --release\
		-Z build-std=core,compiler_builtins \
		-Z build-std-features=compiler-builtins-mem \
		--target $(KERNEL_TARGET)

# ── Compilation C ─────────────────────────────────────────────────
kernel_c.o: $(SRCS_C)
	$(CC) $(CFLAGS) -c $< -o $@

# ── Assemblage boot (commun) ──────────────────────────────────────
boot.o: $(SRCS_ASM)
	$(ASM) $(ASMFLAGS) $< -o $@

# ── Link ELF ─────────────────────────────────────────────────────
ifeq ($(LANG),rs)
$(KERNEL_ELF): boot.o $(KERNEL_LIB)
	mkdir -p $(OUTDIR_LANG)
	$(CC) $(CFLAGS) -T $(LINKER) -o $@ boot.o -Wl,--whole-archive $(KERNEL_LIB) -Wl,--no-whole-archive
else
$(KERNEL_ELF): $(OBJS)
	mkdir -p $(OUTDIR_LANG)
	ld -m elf_i386 \
	   -T $(LINKER) \
	   --oformat elf32-i386 \
	   -o $@ \
	   $^
endif

# ── ISO GRUB ─────────────────────────────────────────────────────
iso: $(KERNEL_ELF)
	mkdir -p $(ISODIR_LANG)/boot/grub
	cp $(KERNEL_ELF) $(ISODIR_LANG)/boot/kernel.elf
	printf 'set timeout=0\nset default=0\nmenuentry "MyKernel ($(LANG))" {\n  multiboot /boot/kernel.elf\n}\n' \
		> $(ISODIR_LANG)/boot/grub/grub.cfg
	grub-mkrescue -o $(KERNEL_ISO) $(ISODIR_LANG)

# ── Nettoyage ─────────────────────────────────────────────────────
clean:
	rm -f *.o

fclean: clean
	rm -rf $(OUTDIR) $(ISODIR)
	cd $(KERNEL_CRATE) && cargo clean

re: fclean all