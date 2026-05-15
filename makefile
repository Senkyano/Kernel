NAME := kernel-builder
PWD  := $(shell pwd)

.PHONY: all docker build-c build-rs launch-c-simple launch-c launch-rs-simple launch-rs clean fclean

all: clean build-c build-rs

docker:
	docker build -t $(NAME) .

# ── Compilation ───────────────────────────────────────────────────
build-c:
	docker run --rm \
		-e LANG=c \
		-v "$(PWD)/srcs:/kernel/srcs" \
		-v "$(PWD)/output:/kernel/output" \
		$(NAME)

build-rs:
	docker run --rm \
		-e LANG=rs \
		-v "$(PWD)/srcs:/kernel/srcs" \
		-v "$(PWD)/output:/kernel/output" \
		$(NAME)

# ── Lancement QEMU ────────────────────────────────────────────────
launch-c-simple:
	qemu-system-i386 -kernel output/c/kernel.elf

launch-c:
	qemu-system-i386 -cdrom output/c/kernel.iso -boot d

launch-rs-simple:
	qemu-system-i386 -kernel output/rust/kernel.elf

launch-rs:
	qemu-system-i386 -cdrom output/rust/kernel.iso -boot d

size-c:
	ls -lh output/c/kernel.iso
	stat output/c/kernel.iso

# ── Nettoyage ─────────────────────────────────────────────────────
clean:
	rm -rf output/ isodir/
	docker run --rm \
		-v "$(PWD)/srcs:/kernel/srcs" \
		$(NAME) sh -c "rm -f /kernel/srcs/*.o && cd /kernel/srcs/kernelspace_rust && cargo clean"

fclean: clean
	docker rmi $(NAME)