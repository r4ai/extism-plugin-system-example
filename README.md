```sh
# Build plugin
cd sample-plugin
cargo build --release --target wasm32-wasi
mv target/wasm32-wasi/release/sample_plugin.wasm ../plugin.wasm

# Run app (load plugin.wasm and call `greet` function)
cargo run
# >>> Hello, WebAssembly!
```
