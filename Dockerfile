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
    curl \
	ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Rustup en nightly (nécessaire pour no_std bare-metal)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --default-toolchain nightly \
    && /root/.cargo/bin/rustup component add rust-src llvm-tools-preview
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /kernel
COPY . .

# LANG peut être surchargé au docker run : -e LANG=rs
ENV LANG=c

CMD ["sh", "-c", "make -C srcs LANG=${LANG}"]