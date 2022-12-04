FROM rust:alpine as builder
RUN apk add --no-cache musl-dev upx clang mold openssl --repository=https://dl-cdn.alpinelinux.org/alpine/v3.17/community --repository=https://dl-cdn.alpinelinux.org/alpine/v3.17/main

WORKDIR /build
COPY . .
RUN RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=/usr/bin/mold" cargo build --release -p backend
RUN upx --best --lzma ./target/release/backend

FROM alpine

WORKDIR /app

RUN addgroup -S backend && adduser -S backend -G backend
COPY --from=builder /build/target/release/backend /app/backend

RUN chmod 550 /app/backend
RUN chown backend:backend /app/backend
USER backend:backend

ENV ROCKET_ADDRESS=0.0.0.0

EXPOSE 8000
LABEL org.opencontainers.image.source = "https://github.com/theNTcompany/send-me-money-2"

ENTRYPOINT ["/app/backend"]
