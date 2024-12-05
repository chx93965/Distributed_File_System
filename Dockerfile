# Use the official rust image as a parent image
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /usr/src/dfs

# Install additional necessary tools (optional)
# openssl-dev
# musl-dev
# libssl-dev
# libm
# libgcc

RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the source code into the container
COPY . .

# Optionally create a dummy main.rs for testing (remove if you copy your own source code)
RUN cargo build --release

# copy the binary to the final image
FROM debian:latest

# Install runtime dependencies, including libc and libm (via libc6)
RUN apt-get update && apt-get install -y \
    libc6 \
    build-essential \
    && rm -rf /var/lib/apt/lists/*
    
COPY --from=builder /usr/src/dfs/target/release/chunk /usr/local/bin/chunk
COPY --from=builder /usr/src/dfs/target/release/master /usr/local/bin/master
COPY --from=builder /usr/src/dfs/target/release/client /usr/local/bin/client


