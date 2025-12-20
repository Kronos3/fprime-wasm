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

## Project Layout

This project consists of Rust crates that are divided into two groups, code-generation and code support.

- `fprime_dictionary`: Code generation. support code to load the F Prime JSON dictionary
- `fprime_build`: Code generation. Responsible for compiling the F Prime JSON dictionary into Rust code for interfacing with the sequencing runtime
- `fprime_core`: The F Prime core library that defines framework primitive traits and types as well as interfaces directly with the sequencing runtime
- `fprime_macros`: Procedural macros for traits defined in `fprime_core`. Does not need to be explicitly imported since `fprime_core` will expose these `#[derive()]` macros
- `example` An example crate showing the minimal project needed to get set up and write sequences
