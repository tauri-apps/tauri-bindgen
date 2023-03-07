#![allow(clippy::all, unused)]
use pretty_assertions::assert_eq;
use std::path::{Path, PathBuf};
use tauri_bindgen_core::Generate;
use tauri_bindgen_gen_guest_js::*;

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
        prettier: false,
        romefmt: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "chars",
        include_str!("../../../tests/codegen/chars.wit"),
    );

    assert_eq!(filename, "chars.js");
    assert_eq!(contents, include_str!("./chars.js"));
}

#[test]
fn convention() {
    let opts = Opts {
        prettier: false,
        romefmt: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "conventions",
        include_str!("../../../tests/codegen/conventions.wit"),
    );

    assert_eq!(filename, "conventions.js");
    assert_eq!(contents, include_str!("./conventions.js"));
}

#[test]
fn empty() {
    let opts = Opts {
        prettier: false,
        romefmt: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "empty",
        include_str!("../../../tests/codegen/empty.wit"),
    );

    assert_eq!(filename, "empty.js");
    assert_eq!(contents, include_str!("./empty.js"));
}

#[test]
fn flags() {
    let opts = Opts {
        prettier: false,
        romefmt: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "flegs",
        include_str!("../../../tests/codegen/flags.wit"),
    );

    assert_eq!(filename, "flegs.js");
    assert_eq!(contents, include_str!("./flegs.js"));
}

#[test]
fn floats() {
    let opts = Opts {
        prettier: false,
        romefmt: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "floats",
        include_str!("../../../tests/codegen/floats.wit"),
    );

    assert_eq!(filename, "floats.js");
    assert_eq!(contents, include_str!("./floats.js"));
}

#[test]
fn integers() {
    let opts = Opts {
        prettier: false,
        romefmt: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "integers",
        include_str!("../../../tests/codegen/integers.wit"),
    );

    assert_eq!(filename, "integers.js");
    assert_eq!(contents, include_str!("./integers.js"));
}

#[test]
fn lists() {
    let opts = Opts {
        prettier: false,
        romefmt: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "lists",
        include_str!("../../../tests/codegen/lists.wit"),
    );

    assert_eq!(filename, "lists.js");
    assert_eq!(contents, include_str!("./lists.js"));
}

#[test]
fn many_arguments() {
    let opts = Opts {
        prettier: false,
        romefmt: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "many-arguments",
        include_str!("../../../tests/codegen/many_arguments.wit"),
    );

    assert_eq!(filename, "many-arguments.js");
    assert_eq!(contents, include_str!("./many-arguments.js"));
}

#[test]
fn multi_return() {
    let opts = Opts {
        prettier: false,
        romefmt: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "multi-return",
        include_str!("../../../tests/codegen/multi_return.wit"),
    );

    assert_eq!(filename, "multi-return.js");
    assert_eq!(contents, include_str!("./multi-return.js"));
}

#[test]
fn records() {
    let opts = Opts {
        prettier: false,
        romefmt: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "records",
        include_str!("../../../tests/codegen/records.wit"),
    );

    assert_eq!(filename, "records.js");
    assert_eq!(contents, include_str!("./records.js"));
}

#[test]
fn simple_functions() {
    let opts = Opts {
        prettier: false,
        romefmt: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "simple-functions",
        include_str!("../../../tests/codegen/simple_functions.wit"),
    );

    assert_eq!(filename, "simple-functions.js");
    assert_eq!(contents, include_str!("./simple-functions.js"));
}

#[test]
fn simple_lists() {
    let opts = Opts {
        prettier: false,
        romefmt: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "simple-lists",
        include_str!("../../../tests/codegen/simple_lists.wit"),
    );

    assert_eq!(filename, "simple-lists.js");
    assert_eq!(contents, include_str!("./simple-lists.js"));
}

#[test]
fn small_anonymous() {
    let opts = Opts {
        prettier: false,
        romefmt: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "small-anonymous",
        include_str!("../../../tests/codegen/small_anonymous.wit"),
    );

    assert_eq!(filename, "small-anonymous.js");
    assert_eq!(contents, include_str!("./small-anonymous.js"));
}

#[test]
fn strings() {
    let opts = Opts {
        prettier: false,
        romefmt: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "strings",
        include_str!("../../../tests/codegen/strings.wit"),
    );

    assert_eq!(filename, "strings.js");
    assert_eq!(contents, include_str!("./strings.js"));
}

#[test]
fn unions() {
    let opts = Opts {
        prettier: false,
        romefmt: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "unions",
        include_str!("../../../tests/codegen/unions.wit"),
    );

    assert_eq!(filename, "unions.js");
    assert_eq!(contents, include_str!("./unions.js"));
}

#[test]
fn variants() {
    let opts = Opts {
        prettier: false,
        romefmt: false,
    };
    let gen = opts.build();

    let (filename, contents) = gen_interface(
        &gen,
        "variants",
        include_str!("../../../tests/codegen/variants.wit"),
    );

    assert_eq!(filename, "variants.js");
    assert_eq!(contents, include_str!("./variants.js"));
}
