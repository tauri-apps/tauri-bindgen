use std::path::Path;

use tauri_bindgen_core::{Files, WorldGenerator};
use tauri_bindgen_gen_guest_rescript::*;

fn gen_world(
    mut gen: Box<dyn WorldGenerator>,
    name: impl AsRef<str>,
    path: impl AsRef<Path>,
) -> (String, String) {
    let world = wit_parser::parse_file(&path).unwrap();
    let world_hash = tauri_bindgen_core::hash::hash_file(path).unwrap();

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
        fmt: true,
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(gen, "chars", "../../tests/codegen/chars.wit");

    assert_eq!(filename, "Chars.res");
    assert_eq!(contents, include_str!("./Chars.res"));
}

#[test]
fn convention() {
    let opts = Opts {
        fmt: true,
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(gen, "conventions", "../../tests/codegen/conventions.wit");

    assert_eq!(filename, "Conventions.res");
    assert_eq!(contents, include_str!("./Conventions.res"));
}

#[test]
fn empty() {
    let opts = Opts {
        fmt: true,
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(gen, "empty", "../../tests/codegen/empty.wit");

    assert_eq!(filename, "Empty.res");
    assert_eq!(contents, include_str!("./Empty.res"));
}

// #[test]
// fn flags() {
//     let opts = Opts {
//         fmt: true,
//         skip: vec![]
//     };
//     let gen = opts.build();

//     let (filename, contents) = gen_world(gen, "flags", "../../tests/codegen/flags.wit");

//     assert_eq!(filename, "flags.res");
//     assert_eq!(contents, include_str!("./flags.res"));
// }

#[test]
fn floats() {
    let opts = Opts {
        fmt: true,
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(gen, "floats", "../../tests/codegen/floats.wit");

    assert_eq!(filename, "Floats.res");
    assert_eq!(contents, include_str!("./Floats.res"));
}

#[test]
fn integers() {
    let opts = Opts {
        fmt: true,
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(gen, "integers", "../../tests/codegen/integers.wit");

    assert_eq!(filename, "Integers.res");
    assert_eq!(contents, include_str!("./Integers.res"));
}

#[test]
fn many_arguments() {
    let opts = Opts {
        fmt: true,
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "many-arguments",
        "../../tests/codegen/many-arguments.wit",
    );

    assert_eq!(filename, "ManyArguments.res");
    assert_eq!(contents, include_str!("./ManyArguments.res"));
}

#[test]
fn multi_return() {
    let opts = Opts {
        fmt: true,
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) =
        gen_world(gen, "multi-return", "../../tests/codegen/multi-return.wit");

    assert_eq!(filename, "MultiReturn.res");
    assert_eq!(contents, include_str!("./MultiReturn.res"));
}

#[test]
fn records() {
    let opts = Opts {
        fmt: true,
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(gen, "records", "../../tests/codegen/records.wit");

    assert_eq!(filename, "Records.res");
    assert_eq!(contents, include_str!("./records.res"));
}

#[test]
fn simple_functions() {
    let opts = Opts {
        fmt: true,
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "simple-functions",
        "../../tests/codegen/simple-functions.wit",
    );

    assert_eq!(filename, "SimpleFunctions.res");
    assert_eq!(contents, include_str!("./SimpleFunctions.res"));
}

#[test]
fn simple_lists() {
    let opts = Opts {
        fmt: true,
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) =
        gen_world(gen, "simple-lists", "../../tests/codegen/simple-lists.wit");

    assert_eq!(filename, "SimpleLists.res");
    assert_eq!(contents, include_str!("./SimpleLists.res"));
}

#[test]
fn small_anonymous() {
    let opts = Opts {
        fmt: true,
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "small-anonymous",
        "../../tests/codegen/small-anonymous.wit",
    );

    assert_eq!(filename, "SmallAnonymous.res");
    assert_eq!(contents, include_str!("./SmallAnonymous.res"));
}

#[test]
fn strings() {
    let opts = Opts {
        fmt: true,
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(gen, "strings", "../../tests/codegen/strings.wit");

    assert_eq!(filename, "Strings.res");
    assert_eq!(contents, include_str!("./strings.res"));
}

#[test]
fn unions() {
    let opts = Opts {
        fmt: true,
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(gen, "unions", "../../tests/codegen/unions.wit");

    assert_eq!(filename, "Unions.res");
    assert_eq!(contents, include_str!("./unions.res"));
}

#[test]
fn variants() {
    let opts = Opts {
        fmt: true,
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(gen, "variants", "../../tests/codegen/variants.wit");

    assert_eq!(filename, "Variants.res");
    assert_eq!(contents, include_str!("./variants.res"));
}
