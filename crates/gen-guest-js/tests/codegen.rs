#![allow(clippy::all, unused)]
use pretty_assertions::assert_eq;
use std::path::{Path, PathBuf};
use tauri_bindgen_core::{Generate, GeneratorBuilder};
use tauri_bindgen_gen_guest_js::Builder;

fn gen_interface(
    opts: Builder,
    _name: impl AsRef<str>,
    input: impl AsRef<str>,
) -> (String, String) {
    let iface = wit_parser::parse_str(&input, |_| false).unwrap();

    let gen = opts.build(iface);
    let (filename, contents) = gen.to_file();

    (filename.to_str().unwrap().to_string(), contents)
}

#[test]
fn chars() {
    let opts = Builder {
        prettier: false,
        romefmt: false,
    };

    let (filename, contents) = gen_interface(
        opts,
        "chars",
        include_str!("../../../wit/chars.wit"),
    );

    assert_eq!(filename, "chars.js");
    assert_eq!(contents, include_str!("./chars.js"));
}

#[test]
fn convention() {
    let opts = Builder {
        prettier: false,
        romefmt: false,
    };

    let (filename, contents) = gen_interface(
        opts,
        "conventions",
        include_str!("../../../wit/conventions.wit"),
    );

    assert_eq!(filename, "conventions.js");
    assert_eq!(contents, include_str!("./conventions.js"));
}

#[test]
fn empty() {
    let opts = Builder {
        prettier: false,
        romefmt: false,
    };

    let (filename, contents) = gen_interface(
        opts,
        "empty",
        include_str!("../../../wit/empty.wit"),
    );

    assert_eq!(filename, "empty.js");
    assert_eq!(contents, include_str!("./empty.js"));
}

#[test]
fn flags() {
    let opts = Builder {
        prettier: false,
        romefmt: false,
    };

    let (filename, contents) = gen_interface(
        opts,
        "flegs",
        include_str!("../../../wit/flags.wit"),
    );

    assert_eq!(filename, "flegs.js");
    assert_eq!(contents, include_str!("./flegs.js"));
}

#[test]
fn floats() {
    let opts = Builder {
        prettier: false,
        romefmt: false,
    };

    let (filename, contents) = gen_interface(
        opts,
        "floats",
        include_str!("../../../wit/floats.wit"),
    );

    assert_eq!(filename, "floats.js");
    assert_eq!(contents, include_str!("./floats.js"));
}

#[test]
fn integers() {
    let opts = Builder {
        prettier: false,
        romefmt: false,
    };

    let (filename, contents) = gen_interface(
        opts,
        "integers",
        include_str!("../../../wit/integers.wit"),
    );

    assert_eq!(filename, "integers.js");
    assert_eq!(contents, include_str!("./integers.js"));
}

#[test]
fn lists() {
    let opts = Builder {
        prettier: false,
        romefmt: false,
    };

    let (filename, contents) = gen_interface(
        opts,
        "lists",
        include_str!("../../../wit/lists.wit"),
    );

    assert_eq!(filename, "lists.js");
    assert_eq!(contents, include_str!("./lists.js"));
}

#[test]
fn many_arguments() {
    let opts = Builder {
        prettier: false,
        romefmt: false,
    };

    let (filename, contents) = gen_interface(
        opts,
        "many-arguments",
        include_str!("../../../wit/many_arguments.wit"),
    );

    assert_eq!(filename, "many-arguments.js");
    assert_eq!(contents, include_str!("./many-arguments.js"));
}

#[test]
fn multi_return() {
    let opts = Builder {
        prettier: false,
        romefmt: false,
    };

    let (filename, contents) = gen_interface(
        opts,
        "multi-return",
        include_str!("../../../wit/multi_return.wit"),
    );

    assert_eq!(filename, "multi-return.js");
    assert_eq!(contents, include_str!("./multi-return.js"));
}

#[test]
fn records() {
    let opts = Builder {
        prettier: false,
        romefmt: false,
    };

    let (filename, contents) = gen_interface(
        opts,
        "records",
        include_str!("../../../wit/records.wit"),
    );

    assert_eq!(filename, "records.js");
    assert_eq!(contents, include_str!("./records.js"));
}

#[test]
fn simple_functions() {
    let opts = Builder {
        prettier: false,
        romefmt: false,
    };

    let (filename, contents) = gen_interface(
        opts,
        "simple-functions",
        include_str!("../../../wit/simple_functions.wit"),
    );

    assert_eq!(filename, "simple-functions.js");
    assert_eq!(contents, include_str!("./simple-functions.js"));
}

#[test]
fn simple_lists() {
    let opts = Builder {
        prettier: false,
        romefmt: false,
    };

    let (filename, contents) = gen_interface(
        opts,
        "simple-lists",
        include_str!("../../../wit/simple_lists.wit"),
    );

    assert_eq!(filename, "simple-lists.js");
    assert_eq!(contents, include_str!("./simple-lists.js"));
}

#[test]
fn small_anonymous() {
    let opts = Builder {
        prettier: false,
        romefmt: false,
    };

    let (filename, contents) = gen_interface(
        opts,
        "small-anonymous",
        include_str!("../../../wit/small_anonymous.wit"),
    );

    assert_eq!(filename, "small-anonymous.js");
    assert_eq!(contents, include_str!("./small-anonymous.js"));
}

#[test]
fn strings() {
    let opts = Builder {
        prettier: false,
        romefmt: false,
    };

    let (filename, contents) = gen_interface(
        opts,
        "strings",
        include_str!("../../../wit/strings.wit"),
    );

    assert_eq!(filename, "strings.js");
    assert_eq!(contents, include_str!("./strings.js"));
}

#[test]
fn unions() {
    let opts = Builder {
        prettier: false,
        romefmt: false,
    };

    let (filename, contents) = gen_interface(
        opts,
        "unions",
        include_str!("../../../wit/unions.wit"),
    );

    assert_eq!(filename, "unions.js");
    assert_eq!(contents, include_str!("./unions.js"));
}

#[test]
fn variants() {
    let opts = Builder {
        prettier: false,
        romefmt: false,
    };

    let (filename, contents) = gen_interface(
        opts,
        "variants",
        include_str!("../../../wit/variants.wit"),
    );

    assert_eq!(filename, "variants.js");
    assert_eq!(contents, include_str!("./variants.js"));
}
