# Gongcheck-Editor

## Pre

- Rust
- `wasm-pack`: install `cargo install wasm-pack`

## Build

1. Check `Cargo.toml` that has assign `crate-type` as `"cdylib"`.

```rust
[lib]
crate-type = ["cdylib"]
```

2. Set build target as `wasm32-unknown-unknown` and build

```shell
$ rustup target add wasm32-unknown-unknown
$ cargo build --target wasm32-unknown-unknown --release
```

this step will generate `.wasm` file at `/target/wasm32-unknwon-unknown`.

3. 