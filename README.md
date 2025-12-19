# FPrime WASM

This repository includes compile-time and runtime-time
dependencies for interfacing Rust with F Prime running
inside a WASM interpreter.

## Installation

1. Install Rust: https://doc.rust-lang.org/cargo/getting-started/installation.html

2. Get the WASM Rust target:

```shell
rustup target add wasm32-unknown-unknown
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
Using the experimental WASM F Prime deployment, run:

```
R00:00:00 Ref.cmdSeq.RUN "example.wasm" NO_BLOCK
```
