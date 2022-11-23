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

> **Note**: This project is under heavy development and should be considered experimental. I will be doing many sweeping changes, so depending on it is not recommened at this point. If you are excited by this though, and want help out, please feel free to test it out, create issues, and open PRs!

This project generates bindings for **both sides** of the Taur IPC bridge. Bindings are declared using [`*.wit`](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md) files that describe exposed functions and shared types. `tauri-bindgen` can then generate ergonomic wrappers for the host interface (declaring the API the Tauri host must expose) and for a variety of client languages.

This project's main purpose right now is to support the development of Tauri API bindings and plugins, where users might want support for many different languages.

### Rationale

Under the current IPC system, Tauri can make no compile-time guarantees on the correctness of your `invoke` calls and any mistakes will result in nasty runtime errors. For example you might have misspelled a command or parameter name which you will only notice when actually running the app!

`tauri-bindgen` will generate traits and invoke-handlers for the [Host](#host) and [Guest](#guest) bindings for JavaScript, TypeScript, Rust and ReScript from a single, shared source of truth using the [`*.wit`](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md) format as a Interface Definition Language (IDL). The generated bindings automatically take care of the heavy lifting, such as correctly calling the `invoke` function and serializing/deserializing parameters and results.

Here are a few reasons why that is cool:

- **Compile-time Checks**

When using strongly typed languages, such as Rust, TypeScript or ReScript the generated code will automatically ensure that you are calling the API correctly, as long as it passes the type checking youre golden. This is especially neat **when working in a team**, so your colleagues can't just change command signatures and pull the rug out from under you.

- **Easily auditable**

Sometimes in bigger apps it's easy to loose track of all the commands your backend exposes, there might be an old version left somewhere that still get's exposed to the frontend. This is of course a big security risk. Having all possible commands be declared in a single place makes them easily auditable for you and possibly external security auditing professionals.

- **Automatic documentation**

`tauri-bindgen` can also generate easy to read markdown and html descriptions of your interfaces for documentation purposes.

- **Future proof**

We're planning a big rework of the IPC bridge in the coming months and `tauri-bindgen` has been designed with some of the early design-work in mind. A big rework of course means breaking changes, but using tauri bindgen isolates from these changes, just update Tauri, update `tauri-bindgen`, and re-generate your bindings and your code will continue to work!

## Terminology

### **`Host`**

Host refers to the Tauri rust core, so the place where your commands are implemented.

### **`Guest`**

Guest refers to the code running in the Webview, this will usually be written in JavaScript/TypeScript, but can also be written in any other language that runs on the Web (through WASM) like Rust, C, or Swift.

## Supported Bindings

- **Host** - Generates a trait declaration and an `invoke_handler` from the interface. This generator can also be used through the `tauri-bindgen-host` crate (located at `crates/host`) and, has a generate! macro for generating code.
- **Guest JavaScript** - Generates a module exposing all functions, functions internally know how to serialize and invoke their host counterpart.
- **Guest Typescript** - The same as the JavaScript guest, but generates Typescript files.
- **Guest Rust** - Generates bindings using `wasm_bindgen` that can be used in Rust compile-to-wasm frontend frameworks such as sycamore. You probably want to depend on the `tauri-bindgen-guest-rust` crate (located at `crates/guest-rust`) and use the `generate!` macro to generate code.
- **Guest ReScript** - Generates bindings for the ReScript language.
- **Markdown** - Generates a markdown description of the interface for documentation purposes.

## Example

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

and lastly generate client bindings, this can be done for JavaScript, Typescript or ReScript using the following commands:

```
tauri-bindgen guest javascript greet.wit --prettier // we can run prettier on the generated code. requires a global prettier install
tauri-bindgen guest typescript greet.wit --prettier
tauri-bindgen guest rescript greet.wit --fmt // run `rescript format` on the generated code. requires a global rescript install
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

## Contributing

PRs are welcome!

## Credits

This project has been inspired by and based off of the awesome [`wit-bindgen`](https://github.com/bytecodealliance/wit-bindgen) tool for WebAseembly by the [Bytecode Alliance](https://bytecodealliance.org).

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
