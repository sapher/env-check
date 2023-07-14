FROM rust:slim-buster as builder

WORKDIR /opt

COPY . .

RUN rustup target add x86_64-unknown-linux-musl \
    && cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest

COPY --from=builder /opt/target/x86_64-unknown-linux-musl/release/env-checks /usr/local/bin/env-checks

RUN env-checks --version