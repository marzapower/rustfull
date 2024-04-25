FROM rust:1.77.2 AS builder
WORKDIR /build
RUN apt update && apt install -y musl-tools musl-dev openssl libssl-dev
RUN update-ca-certificates
RUN rustup target add x86_64-unknown-linux-musl

# This is the macOS part
RUN apt-get install -y g++-aarch64-linux-gnu libc6-dev-arm64-cross
RUN rustup target add aarch64-unknown-linux-gnu


COPY . .
# RUN --mount=type=cache,target=/usr/local/cargo/registry \
#     --mount=type=cache,target=/build/target \
#     cargo build --target x86_64-unknown-linux-musl --release && cp /build/target/x86_64-unknown-linux-musl/release/rustfull /build/rustfull

# This is the macOS part
RUN export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/target \
    cargo build --target=aarch64-unknown-linux-gnu --release && cp /build/target/aarch64-unknown-linux-gnu/release/rustfull /build/rustfull

FROM alpine:3.19 AS runtime
COPY --from=builder /build/rustfull /bin/rustfull
CMD ["rustfull"]
