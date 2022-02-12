# Builder
FROM docker.io/library/rust:1.58-alpine3.15 AS builder
WORKDIR /usr/src
COPY . .
RUN apk --no-cache add libc-dev && \
    cargo install --target x86_64-unknown-linux-musl --path .

# Clean image
FROM scratch
COPY --from=builder /usr/local/cargo/bin/erro-rs /usr/bin/erro-rs
CMD ["erro-rs"]
EXPOSE 3000
