use std::path::Path;

use tauri_bindgen_core::{Files, WorldGenerator};
use tauri_bindgen_gen_guest_ts::*;
use wit_parser::World;

fn gen_world(mut gen: Box<dyn WorldGenerator>, name: impl AsRef<str>, path: impl AsRef<Path>) -> (String, String) {
    let world = World::parse_file(path).unwrap();

    let mut files = Files::default();

    gen.generate(name.as_ref(), &world, &mut files);

    let (filename, contents) = files.iter().next().unwrap();

    (filename.to_string(), std::str::from_utf8(contents).unwrap().to_string())
}

#[test]
fn chars() {
    let opts = Opts {
        prettier: true,
        skip: vec![]
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(gen, "chars", "../../tests/codegen/chars.wit");

    assert_eq!(filename, "chars.ts");
    assert_eq!(contents, include_str!("./chars.ts"));
}

#[test]
fn convention() {
    let opts = Opts {
        prettier: true,
        skip: vec![]
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(gen, "conventions", "../../tests/codegen/conventions.wit");

    assert_eq!(filename, "conventions.ts");
    assert_eq!(contents, include_str!("./conventions.ts"));
}

#[test]
fn empty() {
    let opts = Opts {
        prettier: true,
        skip: vec![]
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(gen, "empty", "../../tests/codegen/empty.wit");

    assert_eq!(filename, "empty.ts");
    assert_eq!(contents, include_str!("./empty.ts"));
}

#[test]
fn flags() {
    let opts = Opts {
        prettier: true,
        skip: vec![]
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(gen, "flags", "../../tests/codegen/flags.wit");

    assert_eq!(filename, "flags.ts");
    assert_eq!(contents, include_str!("./flags.ts"));
}

#[test]
fn floats() {
    let opts = Opts {
        prettier: true,
        skip: vec![]
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(gen, "floats", "../../tests/codegen/floats.wit");

    assert_eq!(filename, "floats.ts");
    assert_eq!(contents, include_str!("./floats.ts"));
}

#[test]
fn integers() {
    let opts = Opts {
        prettier: true,
        skip: vec![]
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(gen, "integers", "../../tests/codegen/integers.wit");

    assert_eq!(filename, "integers.ts");
    assert_eq!(contents, include_str!("./integers.ts"));
}

#[test]
fn many_arguments() {
    let opts = Opts {
        prettier: true,
        skip: vec![]
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(gen, "many-arguments", "../../tests/codegen/many-arguments.wit");

    assert_eq!(filename, "many-arguments.ts");
    assert_eq!(contents, include_str!("./many-arguments.ts"));
}

#[test]
fn multi_return() {
    let opts = Opts {
        prettier: true,
        skip: vec![]
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(gen, "multi-return", "../../tests/codegen/multi-return.wit");

    assert_eq!(filename, "multi-return.ts");
    assert_eq!(contents, include_str!("./multi-return.ts"));
}

#[test]
fn records() {
    let opts = Opts {
        prettier: true,
        skip: vec![]
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(gen, "records", "../../tests/codegen/records.wit");

    assert_eq!(filename, "records.ts");
    assert_eq!(contents, include_str!("./records.ts"));
}

#[test]
fn simple_functions() {
    let opts = Opts {
        prettier: true,
        skip: vec![]
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(gen, "simple-functions", "../../tests/codegen/simple-functions.wit");

    assert_eq!(filename, "simple-functions.ts");
    assert_eq!(contents, include_str!("./simple-functions.ts"));
}

#[test]
fn simple_lists() {
    let opts = Opts {
        prettier: true,
        skip: vec![]
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(gen, "simple-lists", "../../tests/codegen/simple-lists.wit");

    assert_eq!(filename, "simple-lists.ts");
    assert_eq!(contents, include_str!("./simple-lists.ts"));
}

#[test]
fn small_anonymous() {
    let opts = Opts {
        prettier: true,
        skip: vec![]
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(gen, "small-anonymous", "../../tests/codegen/small-anonymous.wit");

    assert_eq!(filename, "small-anonymous.ts");
    assert_eq!(contents, include_str!("./small-anonymous.ts"));
}

#[test]
fn strings() {
    let opts = Opts {
        prettier: true,
        skip: vec![]
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(gen, "strings", "../../tests/codegen/strings.wit");

    assert_eq!(filename, "strings.ts");
    assert_eq!(contents, include_str!("./strings.ts"));
}

#[test]
fn unions() {
    let opts = Opts {
        prettier: true,
        skip: vec![]
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(gen, "unions", "../../tests/codegen/unions.wit");

    assert_eq!(filename, "unions.ts");
    assert_eq!(contents, include_str!("./unions.ts"));
}

#[test]
fn variants() {
    let opts = Opts {
        prettier: true,
        skip: vec![]
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(gen, "variants", "../../tests/codegen/variants.wit");

    assert_eq!(filename, "variants.ts");
    assert_eq!(contents, include_str!("./variants.ts"));
}