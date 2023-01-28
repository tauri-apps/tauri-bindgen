# General

- improve error DX
- structured serialization for results
- Validate serialization and deserialization works across the bridge
- add logging across the board, both during gen and runtime
- fork `wit-parser` and remove worlds & exports
- publish to crates.io

# CLI

- fix out-dir option
- add verbose option
- add napi bindings

# Host

- improve host error type
- add optional tracing to `gen-host`
- improve integration with Tauri (possibly through a WIP impl of the new IPC routing system?)

# Guest Rust

- handle errors even when return type is not result optionally disabled through unchecked
- reduce reliance on extern crates in rust runtime code (`wasm_bindgen` and `serde`).
- ~~Move `wasm_bindgen` macro use into guest-rust and only use exported `async fn send<T: Serialize>(T)` function~~

# Guest JavaScript

- fix serialization of number array types
- How should `Result` types be represented in JavaScript? Should errors be flattened and thrown indiscriminately or should they be represented as objects?
- better representation for general variant types

# Guest Typescript

- Better way to represent `Result` and `Option` than the current rather clunky way
- Adopt the same handling of error types as the JavaScript guest
- better representation for general variant types

# Roadmap

- `rollup-plugin-tauri-bindgen`
- switch `tauri-sys` and try implementing the Tauri API
- explore binary serialization of types instead of JSON
- explore adding the `stream` type defined by wit
- explore adding back exports (functions exposed from JS to Rust)
- explore platform specific codegen