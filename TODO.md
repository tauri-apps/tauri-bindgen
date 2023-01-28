# General

- add logging across the board, both during gen and runtime
- publish to crates.io

# CLI

- add verbose option
- add napi bindings

# Host

- improve host error type
- improve integration with Tauri (possibly through a WIP impl of the new IPC routing system?)

# Guest Rust

- handle errors even when return type is not result optionally disabled through unchecked
- reduce reliance on extern crates in rust runtime code (`wasm_bindgen` and `serde`).

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
- explore adding the `stream` type defined by wit
- explore adding back exports (functions exposed from JS to Rust)
- explore platform specific codegen