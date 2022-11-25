FROM rust:alpine as builder
RUN apk add --no-cache musl-dev upx

WORKDIR /build
COPY . .
RUN cargo build --release
RUN upx --best --lzma ./target/release/backend

FROM alpine

WORKDIR /app

RUN addgroup -S backend && adduser -S backend -G backend
COPY --from=builder --chown=backend:backend --chmod=550 /build/target/release/backend /app/backend

USER backend:backend

ENV ROCKET_ADDRESS=0.0.0.0

EXPOSE 8000

ENTRYPOINT ["/app/backend"]
