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

## Writing sequences

A sequence is an ordinary Rust function. `#[fprime_main]` marks the function
the runtime invokes and also enables a passthrough DSL that lets a command
argument name an enumerated constant on its own; a helper function that is not
the entry point enables just the DSL with `#[fprime]`:

```rust
#[fprime_main]
pub fn main() {
    CdhCore.events.SET_EVENT_FILTER(ACTIVITY_HI, DISABLED);
    Ref.dpDemo.Dp(IMMEDIATE, 0, PROC_TYPE_NONE);
}
```

The receiver chain names the command in the dictionary, the command's formal
parameters give the expected type of every argument, and the expansion prefixes
each constant with the path of the enum that declares it:

```rust
Ref.dpDemo.Dp(
    crate::Defs::Ref::DpDemo::DpReqType::IMMEDIATE,
    0,
    crate::Defs::Fw::DpCfg::ProcType::PROC_TYPE_NONE,
);
```

Because the expected type selects the enum, a constant name shared by several
enums is never ambiguous. `DISABLED` belongs to four enums in the `Ref`
dictionary and `IMMEDIATE` to two, and neither needs a `use` or a hint at the
call site. Struct literals and array literals are resolved the same way, one
level at a time:

```rust
Ref.typeDemo.CHOICE_PAIR(ChoicePair { firstChoice: RED, secondChoice: TWO });
Ref.typeDemo.CHOICES([RED, BLUE]);
```

Everything the dictionary does not describe is left exactly as written, so
locals, constants of your own and explicitly qualified paths keep working. Two
consequences are worth knowing:

- The DSL only applies to command arguments, where a parameter type is known.
  Elsewhere -- a `let` binding, say -- write the path out.
- Where it does apply, the parameter type wins. A constant brought into scope
  with `use` is still replaced by the dictionary's path for that parameter.

The dictionary comes from the `fprime_build::generate` call in the crate's
`build.rs`, which publishes it to the macros through the `FPRIME_DICTIONARY`
environment variable. `#[fprime]` takes no arguments; passing one is an error rather than
being ignored.
