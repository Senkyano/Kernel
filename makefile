NAME := kernel-builder

all: docker docker-up

docker:
	docker build -t $(NAME) .

docker-up:
	docker run --rm \
	-v "$(PWD)/srcs:/kernel/srcs" \
	-v "$(PWD)/output:/kernel/output" \
	-v "$(PWD)/linker.ld:/kernel/linker.ld" \
	kernel-builder

launch-simple:
	qemu-system-i386 -kernel output/kernel.elf

launch-qemu:
	qemu-system-i386 -cdrom output/kernel.iso -boot d

clean:
	docker rmi $(NAME)