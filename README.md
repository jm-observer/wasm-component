# rust-lib

client-expand-rs(cargo-expand出来的代码)不对，无法编译
```
cargo build --target wasm32-wasi --release --package client-expand
wasm-tools component new ./target/wasm32-wasi/release/client_expand.wasm -o client-expand.wasm --adapt ./wasi_snapshot_preview1.reactor.wasm
```


