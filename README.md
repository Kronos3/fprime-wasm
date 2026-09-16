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
cargo build --release
```

The Wasm target comes from `example/.cargo/config.toml`, so `--target` is not
needed. This generates one binary per sequence under
`target/wasm32v1-none/release/`, e.g. `example.wasm` and `no_op.wasm`.

### Adding a sequence

Each sequence is its own bin, so each one builds to its own `.wasm`:

1. Add `example/src/bin/<name>.rs`:

   ```rust
   #![no_std]
   #![no_main]

   use example::*;
   use fprime_core::*;

   #[fprime_main]
   pub fn main() {
       CdhCore.cmdDisp.CMD_NO_OP();
   }
   ```

   The `use example::*;` glob has to sit at the crate root: the sequencing DSL
   rewrites bare enumerated constants into `crate::Defs::...` paths, and that
   glob is what makes `Defs` resolve.

2. Declare it in `example/Cargo.toml` so Cargo does not try to build a test
   harness for a `#![no_main]` bin:

   ```toml
   [[bin]]
   name = "<name>"
   test = false
   bench = false
   ```

The dictionary is generated once by `build.rs` and shared through the `example`
library target, so extra sequences do not re-expand it.
