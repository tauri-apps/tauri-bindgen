<div align="center">
  <h1>
    <code>tauri-bindgen</code>
  </h1>
  <p>
    <strong>Typesafe language bindings generator for the Tauri IPC bridge</strong>
  </p>
</div>

[![MIT or Apache 2.0 licensed][mit-or-apache-badge]][mit-or-apache-url]

[mit-or-apache-badge]: https://img.shields.io/badge/license-MIT%20or%20Apache%202.0-blue.svg
[mit-or-apache-url]: LICENSE

## About

> **Note**: This project is under heavy development and should be considered experimental.
> I will be doing many sweeping changes, so depending on it is not recommened at this point.
> If you are excited by this though, and want help out, please feel free to test it out, create issues, and open PRs!

This project generates bindings for **both sides** of the Taur IPC bridge. Bindings are declared using [`*.wit`](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md) files that describe exposed functions and shared types. `tauri-bindgen` can then generate ergonomic wrappers for the host interface (declaring the API the Tauri host must expose) and for a variety of client languages.

## Supported Bindings

- **Host** - Generates a trait declaration and an `invoke_handler` from the interface. This generator can also be used through the `tauri-bindgen-host` crate (located at `crates/host`) and, has a generate! macro for generating code.
- **Guest JavaScript** - Generates a module exposing all functions, functions internally know how to serialize and invoke their host counterpart.
- **Guest Typescript** - The same as the JavaScript guest, but generates Typescript files.
- **Guest Rust** - Generates bindings using `wasm_bindgen` that can be used in Rust compile-to-wasm frontend frameworks such as sycamore. You probably want to depend on the `tauri-bindgen-guest-rust` crate (located at `crates/guest-rust`) and use the `generate!` macro to generate code.
- **Markdown** - Generates a markdown description of the interface for documentation purposes.

## Exmple

Declare your interface in a `*.wit` file:

```wit
interface greet {
  greet: func(name: string) -> string
}

world the-world {
  import greet: greet
  export greet: greet
  default export greet
}
```

then you can generate the host bindings:

```rust
use tauri::{
    plugin::{self, TauriPlugin},
    Runtime,
};

tauri_bindgen_host::generate!({
    path: "greet.wit",
    async: false
});

#[derive(Clone, Copy)]
struct Ctx;

impl greet::Greet for Ctx {
    fn greet(&self, name: String) -> tauri_bindgen_host::anyhow::Result<String> {
        Ok(format!("Hello, {}! You've been greeted from code-generated Rust!", name))
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    plugin::Builder::new("greet")
        .invoke_handler(greet::invoke_handler(Ctx))
        .build()
}
```

and lastly generate client bindings, this can be done for JavaScript/Typescript using the following commands:

```
tauri-bindgen guest javascript greet.wit --prettier // we can run prettier on the generated code. requires a global prettier install
tauri-bindgen guest typescript greet.wit --prettier
```

or for rust using the provided `generate!` macro:

```rust
tauri_bindgen_guest_rust::generate!({
    path: "greet.wit"
});

// now we can call greet
let greeting = greet::greet("Jonas").await;
```

see also [the example](./examples/basic/).
