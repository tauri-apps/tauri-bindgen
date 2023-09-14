#![allow(clippy::all, unused)]
use pretty_assertions::assert_eq;
use std::path::{Path, PathBuf};
use tauri_bindgen_core::{Generate, GeneratorBuilder};
use tauri_bindgen_gen_host::Builder;

fn gen_interface(
    opts: Builder,
    _name: impl AsRef<str>,
    input: impl AsRef<str>,
) -> (String, String) {
    let iface = wit_parser::parse_and_resolve_str(&input, |_| false).unwrap();

    let mut gen = opts.build(iface);
    let (filename, contents) = gen.to_file();

    (filename.to_str().unwrap().to_string(), contents)
}

#[test]
fn chars() {
    let opts = Builder {
        fmt: true,
        tracing: true,
        async_: false,
    };

    let (filename, contents) = gen_interface(opts, "chars", include_str!("../../../wit/chars.wit"));

    assert_eq!(filename, "chars.rs");
    assert_eq!(contents, include_str!("./sync/chars.rs"));
}

#[test]
fn convention() {
    let opts = Builder {
        fmt: true,
        tracing: true,
        async_: false,
    };

    let (filename, contents) = gen_interface(
        opts,
        "conventions",
        include_str!("../../../wit/conventions.wit"),
    );

    assert_eq!(filename, "conventions.rs");
    assert_eq!(contents, include_str!("./sync/conventions.rs"));
}

#[test]
fn empty() {
    let opts = Builder {
        fmt: true,
        tracing: true,
        async_: false,
    };

    let (filename, contents) = gen_interface(opts, "empty", include_str!("../../../wit/empty.wit"));

    assert_eq!(filename, "empty.rs");
    assert_eq!(contents, include_str!("./sync/empty.rs"));
}

#[test]
fn flags() {
    let opts = Builder {
        fmt: true,
        tracing: true,
        async_: false,
    };

    let (filename, contents) = gen_interface(opts, "flegs", include_str!("../../../wit/flags.wit"));

    assert_eq!(filename, "flegs.rs");
    assert_eq!(contents, include_str!("./sync/flegs.rs"));
}

#[test]
fn floats() {
    let opts = Builder {
        fmt: true,
        tracing: true,
        async_: false,
    };

    let (filename, contents) =
        gen_interface(opts, "floats", include_str!("../../../wit/floats.wit"));

    assert_eq!(filename, "floats.rs");
    assert_eq!(contents, include_str!("./sync/floats.rs"));
}

#[test]
fn integers() {
    let opts = Builder {
        fmt: true,
        tracing: true,
        async_: false,
    };

    let (filename, contents) =
        gen_interface(opts, "integers", include_str!("../../../wit/integers.wit"));

    assert_eq!(filename, "integers.rs");
    assert_eq!(contents, include_str!("./sync/integers.rs"));
}

#[test]
fn lists() {
    let opts = Builder {
        fmt: true,
        tracing: true,
        async_: false,
    };

    let (filename, contents) = gen_interface(opts, "lists", include_str!("../../../wit/lists.wit"));

    assert_eq!(filename, "lists.rs");
    assert_eq!(contents, include_str!("./sync/lists.rs"));
}

#[test]
fn many_arguments() {
    let opts = Builder {
        fmt: true,
        tracing: true,
        async_: false,
    };

    let (filename, contents) = gen_interface(
        opts,
        "many-arguments",
        include_str!("../../../wit/many_arguments.wit"),
    );

    assert_eq!(filename, "many-arguments.rs");
    assert_eq!(contents, include_str!("./sync/many-arguments.rs"));
}

#[test]
fn multi_return() {
    let opts = Builder {
        fmt: true,
        tracing: true,
        async_: false,
    };

    let (filename, contents) = gen_interface(
        opts,
        "multi-return",
        include_str!("../../../wit/multi_return.wit"),
    );

    assert_eq!(filename, "multi-return.rs");
    assert_eq!(contents, include_str!("./sync/multi-return.rs"));
}

#[test]
fn records() {
    let opts = Builder {
        fmt: true,
        tracing: true,
        async_: false,
    };

    let (filename, contents) =
        gen_interface(opts, "records", include_str!("../../../wit/records.wit"));

    assert_eq!(filename, "records.rs");
    assert_eq!(contents, include_str!("./sync/records.rs"));
}

#[test]
fn simple_functions() {
    let opts = Builder {
        fmt: true,
        tracing: true,
        async_: false,
    };

    let (filename, contents) = gen_interface(
        opts,
        "simple-functions",
        include_str!("../../../wit/simple_functions.wit"),
    );

    assert_eq!(filename, "simple-functions.rs");
    assert_eq!(contents, include_str!("./sync/simple-functions.rs"));
}

#[test]
fn simple_lists() {
    let opts = Builder {
        fmt: true,
        tracing: true,
        async_: false,
    };

    let (filename, contents) = gen_interface(
        opts,
        "simple-lists",
        include_str!("../../../wit/simple_lists.wit"),
    );

    assert_eq!(filename, "simple-lists.rs");
    assert_eq!(contents, include_str!("./sync/simple-lists.rs"));
}

#[test]
fn small_anonymous() {
    let opts = Builder {
        fmt: true,
        tracing: true,
        async_: false,
    };

    let (filename, contents) = gen_interface(
        opts,
        "small-anonymous",
        include_str!("../../../wit/small_anonymous.wit"),
    );

    assert_eq!(filename, "small-anonymous.rs");
    assert_eq!(contents, include_str!("./sync/small-anonymous.rs"));
}

#[test]
fn strings() {
    let opts = Builder {
        fmt: true,
        tracing: true,
        async_: false,
    };

    let (filename, contents) =
        gen_interface(opts, "strings", include_str!("../../../wit/strings.wit"));

    assert_eq!(filename, "strings.rs");
    assert_eq!(contents, include_str!("./sync/strings.rs"));
}

#[test]
fn unions() {
    let opts = Builder {
        fmt: true,
        tracing: true,
        async_: false,
    };

    let (filename, contents) =
        gen_interface(opts, "unions", include_str!("../../../wit/unions.wit"));

    assert_eq!(filename, "unions.rs");
    assert_eq!(contents, include_str!("./sync/unions.rs"));
}

#[test]
fn variants() {
    let opts = Builder {
        fmt: true,
        tracing: true,
        async_: false,
    };

    let (filename, contents) =
        gen_interface(opts, "variants", include_str!("../../../wit/variants.wit"));

    assert_eq!(filename, "variants.rs");
    assert_eq!(contents, include_str!("./sync/variants.rs"));
}
