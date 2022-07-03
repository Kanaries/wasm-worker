# Parallel Raytracing

## for HelloWorld

https://rustwasm.github.io/docs/book/introduction.html

## for wasm-bindgen

https://rustwasm.github.io/wasm-bindgen/introduction.html

https://docs.rs/crate/wasm-bindgen/0.2.8

## for this

see

https://rustwasm.github.io/wasm-bindgen/examples/wasm-in-web-worker.html

https://rustwasm.github.io/wasm-bindgen/examples/raytrace.html and https://github.com/rustwasm/wasm-bindgen/tree/main/examples/raytrace-parallel

for compiling the raytracing example, after install rust and install deps in https://rustwasm.github.io/docs/book/game-of-life/setup.html

```sh
rustup toolchain install nightly
rustup override set nightly
rustup target add wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli

cargo build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/raytrace_parallel.wasm --out-dir . --target no-modules
python3 server.py
```

and then visiting http://localhost:8080 in a browser should run the example!
