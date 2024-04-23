FROM rust:1.77.2 AS builder
WORKDIR /build
RUN apt update && apt install -y musl-tools musl-dev openssl libssl-dev
RUN update-ca-certificates
RUN rustup target add x86_64-unknown-linux-musl
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/target \
    cargo build --target x86_64-unknown-linux-musl --release && cp /build/target/x86_64-unknown-linux-musl/release/rustfull /build/rustfull

FROM alpine:3.18 AS runtime
COPY --from=builder /build/rustfull /bin/rustfull
CMD ["rustfull"]
