FROM rust:alpine as builder
RUN apk add --no-cache musl-dev upx clang mold openssl --repository=https://dl-cdn.alpinelinux.org/alpine/v3.17/community --repository=https://dl-cdn.alpinelinux.org/alpine/v3.17/main
RUN RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=/usr/bin/mold" cargo install --locked trunk

RUN rustup toolchain install nightly && \
    rustup default nightly && \
    rustup target add wasm32-unknown-unknown

WORKDIR /build
COPY . .
WORKDIR /build/apps/frontend
RUN trunk build

FROM nginx:stable-alpine

COPY --from=builder /build/apps/frontend/dist /usr/share/nginx/html
COPY ./apps/frontend/nginx.conf /etc/nginx/nginx.conf

EXPOSE 80
LABEL org.opencontainers.image.source = "https://github.com/theNTcompany/send-me-money-2"

CMD ["nginx", "-g", "daemon off;"]

