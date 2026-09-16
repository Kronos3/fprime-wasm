# F Prime Wasm

This repository includes compile-time and runtime-time
dependencies for interfacing Rust with F Prime running
inside a Wasm interpreter.

## Installation

1. Install Rust: https://doc.rust-lang.org/cargo/getting-started/installation.html

2. Get the Wasm Rust target:

```shell
rustup target add wasm32v1-none
```

## Building

This repository includes an example project with works with the `Ref` fprime deployment.

To build, run:
```shell
cd example
cargo build --target wasm32-unknown-unknown --release
```

This will generate a binary in `target/wasm32-unknown-unknown/release/example.wasm`.

## Running
Using the experimental Wasm F Prime deployment, run:

```
R00:00:00 Ref.cmdSeq.RUN "example.wasm" NO_BLOCK
```
