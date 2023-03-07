#![allow(clippy::all, unused)]
use std::path::Path;

use tauri_bindgen_core::{Files, WorldGenerator};
use tauri_bindgen_gen_host::*;

fn gen_world(
    mut gen: Box<dyn WorldGenerator>,
    name: impl AsRef<str>,
    input: impl AsRef<str>,
) -> (String, String) {
    let world = wit_parser::parse_str(&input, |_| false).unwrap();
    let world_hash = tauri_bindgen_core::hash::hash_str(&input).unwrap();

    let mut files = Files::default();

    gen.generate(name.as_ref(), &world, &mut files, &world_hash);

    let (filename, contents) = files.iter().next().unwrap();

    (
        filename.to_string(),
        std::str::from_utf8(contents).unwrap().to_string(),
    )
}

#[test]
fn chars() {
    let opts = Opts {
        fmt: false,
        tracing: true,
        async_: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "chars",
        include_str!("../../../tests/codegen/chars.wit"),
    );

    assert_eq!(filename, "chars.rs");
    assert_eq!(contents, include_str!("./chars.rs"));
}

#[test]
fn convention() {
    let opts = Opts {
        fmt: false,
        tracing: true,
        async_: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "conventions",
        include_str!("../../../tests/codegen/conventions.wit"),
    );

    assert_eq!(filename, "conventions.rs");
    assert_eq!(contents, include_str!("./conventions.rs"));
}

#[test]
fn empty() {
    let opts = Opts {
        fmt: false,
        tracing: true,
        async_: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "empty",
        include_str!("../../../tests/codegen/empty.wit"),
    );

    assert_eq!(filename, "empty.rs");
    assert_eq!(contents, include_str!("./empty.rs"));
}

#[test]
fn flags() {
    let opts = Opts {
        fmt: false,
        tracing: true,
        async_: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "flegs",
        include_str!("../../../tests/codegen/flags.wit"),
    );

    assert_eq!(filename, "flegs.rs");
    assert_eq!(contents, include_str!("./flegs.rs"));
}

#[test]
fn floats() {
    let opts = Opts {
        fmt: false,
        tracing: true,
        async_: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "floats",
        include_str!("../../../tests/codegen/floats.wit"),
    );

    assert_eq!(filename, "floats.rs");
    assert_eq!(contents, include_str!("./floats.rs"));
}

#[test]
fn integers() {
    let opts = Opts {
        fmt: false,
        tracing: true,
        async_: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "integers",
        include_str!("../../../tests/codegen/integers.wit"),
    );

    assert_eq!(filename, "integers.rs");
    assert_eq!(contents, include_str!("./integers.rs"));
}

#[test]
fn lists() {
    let opts = Opts {
        fmt: false,
        tracing: true,
        async_: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "lists",
        include_str!("../../../tests/codegen/lists.wit"),
    );

    assert_eq!(filename, "lists.rs");
    assert_eq!(contents, include_str!("./lists.rs"));
}

#[test]
fn many_arguments() {
    let opts = Opts {
        fmt: false,
        tracing: true,
        async_: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "many-arguments",
        include_str!("../../../tests/codegen/many-arguments.wit"),
    );

    assert_eq!(filename, "many-arguments.rs");
    assert_eq!(contents, include_str!("./many-arguments.rs"));
}

#[test]
fn multi_return() {
    let opts = Opts {
        fmt: false,
        tracing: true,
        async_: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "multi-return",
        include_str!("../../../tests/codegen/multi-return.wit"),
    );

    assert_eq!(filename, "multi-return.rs");
    assert_eq!(contents, include_str!("./multi-return.rs"));
}

#[test]
fn records() {
    let opts = Opts {
        fmt: false,
        tracing: true,
        async_: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "records",
        include_str!("../../../tests/codegen/records.wit"),
    );

    assert_eq!(filename, "records.rs");
    assert_eq!(contents, include_str!("./records.rs"));
}

#[test]
fn simple_functions() {
    let opts = Opts {
        fmt: false,
        tracing: true,
        async_: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "simple-functions",
        include_str!("../../../tests/codegen/simple-functions.wit"),
    );

    assert_eq!(filename, "simple-functions.rs");
    assert_eq!(contents, include_str!("./simple-functions.rs"));
}

#[test]
fn simple_lists() {
    let opts = Opts {
        fmt: false,
        tracing: true,
        async_: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "simple-lists",
        include_str!("../../../tests/codegen/simple-lists.wit"),
    );

    assert_eq!(filename, "simple-lists.rs");
    assert_eq!(contents, include_str!("./simple-lists.rs"));
}

#[test]
fn small_anonymous() {
    let opts = Opts {
        fmt: false,
        tracing: true,
        async_: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "small-anonymous",
        include_str!("../../../tests/codegen/small-anonymous.wit"),
    );

    assert_eq!(filename, "small-anonymous.rs");
    assert_eq!(contents, include_str!("./small-anonymous.rs"));
}

#[test]
fn strings() {
    let opts = Opts {
        fmt: false,
        tracing: true,
        async_: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "strings",
        include_str!("../../../tests/codegen/strings.wit"),
    );

    assert_eq!(filename, "strings.rs");
    assert_eq!(contents, include_str!("./strings.rs"));
}

#[test]
fn unions() {
    let opts = Opts {
        fmt: false,
        tracing: true,
        async_: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "unions",
        include_str!("../../../tests/codegen/unions.wit"),
    );

    assert_eq!(filename, "unions.rs");
    assert_eq!(contents, include_str!("./unions.rs"));
}

#[test]
fn variants() {
    let opts = Opts {
        fmt: false,
        tracing: true,
        async_: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "variants",
        include_str!("../../../tests/codegen/variants.wit"),
    );

    assert_eq!(filename, "variants.rs");
    assert_eq!(contents, include_str!("./variants.rs"));
}
