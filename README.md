# Pallium

The user interface to the logtopus server. 

## Development

First install the Rust wasm toolchain

```
rustup target add wasm32-unknown-unknown
```

See the [yew](https://crates.rs/crates/yew) docs for further toolchain options.

`yew` uses `cargo-web` to start a development server that provides on-the-fly recompilation. To install `cargo-web`, just run:

```
cargo install cargo-web
```

Now, in order to build and start the project:

```
cargo web start --target=wasm32-unknown-unknown
```

Please note that specifying the target is required, since by default `wasm32-unknown-emscripten` will be used.
