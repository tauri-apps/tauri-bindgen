use tauri_bindgen_core::{Files, WorldGenerator};
use tauri_bindgen_gen_guest_js::Opts;

fn gen_world(
    mut gen: Box<dyn WorldGenerator>,
    name: impl AsRef<str>,
    input: impl AsRef<str>,
) -> (String, String) {
    let world = wit_parser::parse_str(&input).unwrap();
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
        prettier: false,
        romefmt: false,
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
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
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
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
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
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
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "flags",
        include_str!("../../../tests/codegen/flags.wit"),
    );

    assert_eq!(filename, "flags.js");
    assert_eq!(contents, include_str!("./flegs.js"));
}

#[test]
fn floats() {
    let opts = Opts {
        prettier: false,
        romefmt: false,
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
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
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "integers",
        include_str!("../../../tests/codegen/integers.wit"),
    );

    assert_eq!(filename, "integers.js");
    assert_eq!(contents, include_str!("./integers.js"));
}

#[test]
fn many_arguments() {
    let opts = Opts {
        prettier: false,
        romefmt: false,
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "many-arguments",
        include_str!("../../../tests/codegen/many-arguments.wit"),
    );

    assert_eq!(filename, "many-arguments.js");
    assert_eq!(contents, include_str!("./many-arguments.js"));
}

#[test]
fn multi_return() {
    let opts = Opts {
        prettier: false,
        romefmt: false,
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "multi-return",
        include_str!("../../../tests/codegen/multi-return.wit"),
    );

    assert_eq!(filename, "multi-return.js");
    assert_eq!(contents, include_str!("./multi-return.js"));
}

#[test]
fn records() {
    let opts = Opts {
        prettier: false,
        romefmt: false,
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
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
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "simple-functions",
        include_str!("../../../tests/codegen/simple-functions.wit"),
    );

    assert_eq!(filename, "simple-functions.js");
    assert_eq!(contents, include_str!("./simple-functions.js"));
}

#[test]
fn simple_lists() {
    let opts = Opts {
        prettier: false,
        romefmt: false,
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "simple-lists",
        include_str!("../../../tests/codegen/simple-lists.wit"),
    );

    assert_eq!(filename, "simple-lists.js");
    assert_eq!(contents, include_str!("./simple-lists.js"));
}

#[test]
fn small_anonymous() {
    let opts = Opts {
        prettier: false,
        romefmt: false,
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "small-anonymous",
        include_str!("../../../tests/codegen/small-anonymous.wit"),
    );

    assert_eq!(filename, "small-anonymous.js");
    assert_eq!(contents, include_str!("./small-anonymous.js"));
}

#[test]
fn strings() {
    let opts = Opts {
        prettier: false,
        romefmt: false,
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
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
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
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
        skip: vec![],
    };
    let gen = opts.build();

    let (filename, contents) = gen_world(
        gen,
        "variants",
        include_str!("../../../tests/codegen/variants.wit"),
    );

    assert_eq!(filename, "variants.js");
    assert_eq!(contents, include_str!("./variants.js"));
}
