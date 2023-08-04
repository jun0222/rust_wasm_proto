# Index

<!-- TOC -->

- [Index](#index)
- [Project setup](#project-setup)
- [Build and run command](#build-and-run-command)

<!-- /TOC -->

# Project setup

```bash
# Install rust
curl --proto https --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.bash_profile
cargo install wasm-pack

# Create project
cargo new --lib rust_wasm_proto
cd rust_wasm_proto

# Build
wasm-pack build --target web

# Run
python3 -m http.server
```

# Build and run command

```bash
# makefileにも定義しているのでmake runで実行可能
wasm-pack build --target web && python3 -m http.server
```
