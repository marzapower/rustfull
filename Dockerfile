ARG PROFILE=release-superopt
ARG PROJECT_NAME
ARG TARGET_ARCHITECTURE=x86_64
# nightly-2024-02-03 is the last 1.77.0 nightly version
ARG RUST_CHANNEL=nightly-2024-02-03 

FROM rust AS builder

ARG PROFILE
ARG PROJECT_NAME
ARG TARGET_ARCHITECTURE
ARG RUST_CHANNEL

WORKDIR /build
RUN apt update && apt install -y musl-tools musl-dev 
RUN update-ca-certificates
RUN rustup toolchain install ${RUST_CHANNEL} --force
RUN rustup default ${RUST_CHANNEL}
RUN rustup target add "${TARGET_ARCHITECTURE}-unknown-linux-musl"
RUN rustup component add rust-src

ENV USER="${PROJECT_NAME}"
ENV UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/target \
    RUSTFLAGS="-Zlocation-detail=none" \
    cargo +${RUST_CHANNEL} build \
    -Z build-std=std,panic_abort \
    -Z build-std-features=panic_immediate_abort \
    --target ${TARGET_ARCHITECTURE}-unknown-linux-musl \
    --profile ${PROFILE} \
    && cp /build/target/${TARGET_ARCHITECTURE}-unknown-linux-musl/${PROFILE}/${PROJECT_NAME} /build/${PROJECT_NAME}

FROM scratch as runtime

ARG PROJECT_NAME

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /rustfull

COPY --from=builder /build/${PROJECT_NAME} /bin/${PROJECT_NAME}
CMD ["${PROJECT_NAME}"]
