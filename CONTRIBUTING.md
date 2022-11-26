# Welcome to the contributing guide
Thank you for being interested in this project ❤!  
Feel free to open an issue, create a PR or help with existing issues / PRs, in this guide you can find everything you
need to do and know to run this app locally.

## Requirements

### Rust nightly
This project is entirely written in rust, which makes this app type safe, and it's docker images small (about 35MB in 
total), so it's fast to pull and start them. You'll also need to install `wasm32-unknown-unknown` target, because
frontend of this challenge is written in wasm.

Preferred way to install rust is with [rustup](https://rustup.rs/).
```bash
rustup toolchain add nightly
rustup default nightly
rustup target add wasm32-unknown-unknown
```

### Trunk
If you're going to be editing the app's frontend, you're going to need to install trunk. It's used to package the
frontend, and it's serve feature can be used to automatically update the app on change.

It can be installed with cargo (rust's package manager). You need to have rust installed ([see Rust nightly](#rust-nightly))
```bash
cargo install --locked trunk
```

### Docker compose
The last thing needed for development is docker and docker compose. It's used to easily start both backend and frontend,
and let them communicate to each other. There are multiple ways to install docker compose. Here are links to it's
installation documentation

- [Docker](https://docs.docker.com/engine/install/)
- [Docker compose](https://docs.docker.com/compose/install/)
