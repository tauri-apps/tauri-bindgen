#![allow(clippy::all, unused)]
use pretty_assertions::assert_eq;
use std::path::{Path, PathBuf};
use tauri_bindgen_core::Generate;
use tauri_bindgen_gen_guest_rust::*;

fn gen_interface(
    mut gen: &dyn Generate,
    _name: impl AsRef<str>,
    input: impl AsRef<str>,
) -> (String, String) {
    let iface = wit_parser::parse_str(&input, |_| false).unwrap();

    let (filename, contents) = gen.to_string(&iface);

    (filename.to_str().unwrap().to_string(), contents)
}

#[test]
fn chars() {
    let opts = Opts {
        fmt: true,
        no_std: false,
        unchecked: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "chars",
        include_str!("../../../tests/codegen/chars.wit"),
    );

    assert_eq!(filename, "chars.rs");
    assert_eq!(contents, include_str!("./chars.rs"));
}

#[test]
fn convention() {
    let opts = Opts {
        fmt: true,
        no_std: false,
        unchecked: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "conventions",
        include_str!("../../../tests/codegen/conventions.wit"),
    );

    assert_eq!(filename, "conventions.rs");
    assert_eq!(contents, include_str!("./conventions.rs"));
}

#[test]
fn empty() {
    let opts = Opts {
        fmt: true,
        no_std: false,
        unchecked: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "empty",
        include_str!("../../../tests/codegen/empty.wit"),
    );

    assert_eq!(filename, "empty.rs");
    assert_eq!(contents, include_str!("./empty.rs"));
}

#[test]
fn flags() {
    let opts = Opts {
        fmt: true,
        no_std: false,
        unchecked: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "flegs",
        include_str!("../../../tests/codegen/flags.wit"),
    );

    assert_eq!(filename, "flegs.rs");
    assert_eq!(contents, include_str!("./flegs.rs"));
}

#[test]
fn floats() {
    let opts = Opts {
        fmt: true,
        no_std: false,
        unchecked: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "floats",
        include_str!("../../../tests/codegen/floats.wit"),
    );

    assert_eq!(filename, "floats.rs");
    assert_eq!(contents, include_str!("./floats.rs"));
}

#[test]
fn integers() {
    let opts = Opts {
        fmt: true,
        no_std: false,
        unchecked: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "integers",
        include_str!("../../../tests/codegen/integers.wit"),
    );

    assert_eq!(filename, "integers.rs");
    assert_eq!(contents, include_str!("./integers.rs"));
}

#[test]
fn lists() {
    let opts = Opts {
        fmt: true,
        no_std: false,
        unchecked: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "lists",
        include_str!("../../../tests/codegen/lists.wit"),
    );

    assert_eq!(filename, "lists.rs");
    assert_eq!(contents, include_str!("./lists.rs"));
}

#[test]
fn many_arguments() {
    let opts = Opts {
        fmt: true,
        no_std: false,
        unchecked: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "many-arguments",
        include_str!("../../../tests/codegen/many_arguments.wit"),
    );

    assert_eq!(filename, "many-arguments.rs");
    assert_eq!(contents, include_str!("./many-arguments.rs"));
}

#[test]
fn multi_return() {
    let opts = Opts {
        fmt: true,
        no_std: false,
        unchecked: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "multi-return",
        include_str!("../../../tests/codegen/multi_return.wit"),
    );

    assert_eq!(filename, "multi-return.rs");
    assert_eq!(contents, include_str!("./multi-return.rs"));
}

#[test]
fn records() {
    let opts = Opts {
        fmt: true,
        no_std: false,
        unchecked: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "records",
        include_str!("../../../tests/codegen/records.wit"),
    );

    assert_eq!(filename, "records.rs");
    assert_eq!(contents, include_str!("./records.rs"));
}

#[test]
fn simple_functions() {
    let opts = Opts {
        fmt: true,
        no_std: false,
        unchecked: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "simple-functions",
        include_str!("../../../tests/codegen/simple_functions.wit"),
    );

    assert_eq!(filename, "simple-functions.rs");
    assert_eq!(contents, include_str!("./simple-functions.rs"));
}

#[test]
fn simple_lists() {
    let opts = Opts {
        fmt: true,
        no_std: false,
        unchecked: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "simple-lists",
        include_str!("../../../tests/codegen/simple_lists.wit"),
    );

    assert_eq!(filename, "simple-lists.rs");
    assert_eq!(contents, include_str!("./simple-lists.rs"));
}

#[test]
fn small_anonymous() {
    let opts = Opts {
        fmt: true,
        no_std: false,
        unchecked: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "small-anonymous",
        include_str!("../../../tests/codegen/small_anonymous.wit"),
    );

    assert_eq!(filename, "small-anonymous.rs");
    assert_eq!(contents, include_str!("./small-anonymous.rs"));
}

#[test]
fn strings() {
    let opts = Opts {
        fmt: true,
        no_std: false,
        unchecked: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "strings",
        include_str!("../../../tests/codegen/strings.wit"),
    );

    assert_eq!(filename, "strings.rs");
    assert_eq!(contents, include_str!("./strings.rs"));
}

#[test]
fn unions() {
    let opts = Opts {
        fmt: true,
        no_std: false,
        unchecked: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "unions",
        include_str!("../../../tests/codegen/unions.wit"),
    );

    assert_eq!(filename, "unions.rs");
    assert_eq!(contents, include_str!("./unions.rs"));
}

#[test]
fn variants() {
    let opts = Opts {
        fmt: true,
        no_std: false,
        unchecked: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "variants",
        include_str!("../../../tests/codegen/variants.wit"),
    );

    assert_eq!(filename, "variants.rs");
    assert_eq!(contents, include_str!("./variants.rs"));
}
