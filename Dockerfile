# Builder
FROM docker.io/library/rust:1.59 AS builder
WORKDIR /usr/src
COPY . .
RUN rustup target add x86_64-unknown-linux-musl && \
    apt update && \
    apt install -y musl-tools musl-dev && \
    rm -rf /var/lib/apt/lists/* && \
    cargo install --target x86_64-unknown-linux-musl --path . && \
    strip -s /usr/local/cargo/bin/erro-rs

# Clean image
FROM scratch
COPY --from=builder /usr/local/cargo/bin/erro-rs /usr/bin/erro-rs
CMD ["erro-rs"]
EXPOSE 3000
