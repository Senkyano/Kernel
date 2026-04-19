FROM debian:bookworm-slim

LABEL description="Cross-Compilation for KernelFromScratch"

RUN apt-get update && apt-get install -y --no-install-recommends \
	nasm \
	gcc \
	gcc-multilib \
	binutils \
	make \
	grub-pc-bin \
	grub-common \
	xorriso \
	mtools \
	&& rm -fr /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly
ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup target add i686-unknown-linux-gnu
RUN rustup component add rust-src

WORKDIR /kernel
COPY . .

CMD [ "make" "-C" "srcs" ]