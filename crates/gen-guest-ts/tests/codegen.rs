use tauri_bindgen_core::GeneratorBuilder;
use tauri_bindgen_gen_guest_ts::Builder;

fn gen_interface(
    opts: Builder,
    _name: impl AsRef<str>,
    input: impl AsRef<str>,
) -> (String, String) {
    let iface = wit_parser::parse_and_resolve_str(&input, |_| false).unwrap();

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

    let (filename, contents) = gen_interface(opts, "chars", include_str!("../../../wit/chars.wit"));

    assert_eq!(filename, "chars.ts");
    assert_eq!(contents, include_str!("./chars.ts"));
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

    assert_eq!(filename, "conventions.ts");
    assert_eq!(contents, include_str!("./conventions.ts"));
}

#[test]
fn empty() {
    let opts = Builder {
        prettier: false,
        romefmt: false,
    };

    let (filename, contents) = gen_interface(opts, "empty", include_str!("../../../wit/empty.wit"));

    assert_eq!(filename, "empty.ts");
    assert_eq!(contents, include_str!("./empty.ts"));
}

#[test]
fn flags() {
    let opts = Builder {
        prettier: false,
        romefmt: false,
    };

    let (filename, contents) = gen_interface(opts, "flegs", include_str!("../../../wit/flags.wit"));

    assert_eq!(filename, "flegs.ts");
    assert_eq!(contents, include_str!("./flegs.ts"));
}

#[test]
fn floats() {
    let opts = Builder {
        prettier: false,
        romefmt: false,
    };

    let (filename, contents) =
        gen_interface(opts, "floats", include_str!("../../../wit/floats.wit"));

    assert_eq!(filename, "floats.ts");
    assert_eq!(contents, include_str!("./floats.ts"));
}

#[test]
fn integers() {
    let opts = Builder {
        prettier: false,
        romefmt: false,
    };

    let (filename, contents) =
        gen_interface(opts, "integers", include_str!("../../../wit/integers.wit"));

    assert_eq!(filename, "integers.ts");
    assert_eq!(contents, include_str!("./integers.ts"));
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

    assert_eq!(filename, "many-arguments.ts");
    assert_eq!(contents, include_str!("./many-arguments.ts"));
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

    assert_eq!(filename, "multi-return.ts");
    assert_eq!(contents, include_str!("./multi-return.ts"));
}

#[test]
fn records() {
    let opts = Builder {
        prettier: false,
        romefmt: false,
    };

    let (filename, contents) =
        gen_interface(opts, "records", include_str!("../../../wit/records.wit"));

    assert_eq!(filename, "records.ts");
    assert_eq!(contents, include_str!("./records.ts"));
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

    assert_eq!(filename, "simple-functions.ts");
    assert_eq!(contents, include_str!("./simple-functions.ts"));
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

    assert_eq!(filename, "simple-lists.ts");
    assert_eq!(contents, include_str!("./simple-lists.ts"));
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

    assert_eq!(filename, "small-anonymous.ts");
    assert_eq!(contents, include_str!("./small-anonymous.ts"));
}

#[test]
fn strings() {
    let opts = Builder {
        prettier: false,
        romefmt: false,
    };

    let (filename, contents) =
        gen_interface(opts, "strings", include_str!("../../../wit/strings.wit"));

    assert_eq!(filename, "strings.ts");
    assert_eq!(contents, include_str!("./strings.ts"));
}

#[test]
fn unions() {
    let opts = Builder {
        prettier: false,
        romefmt: false,
    };

    let (filename, contents) =
        gen_interface(opts, "unions", include_str!("../../../wit/unions.wit"));

    assert_eq!(filename, "unions.ts");
    assert_eq!(contents, include_str!("./unions.ts"));
}

#[test]
fn variants() {
    let opts = Builder {
        prettier: false,
        romefmt: false,
    };

    let (filename, contents) =
        gen_interface(opts, "variants", include_str!("../../../wit/variants.wit"));

    assert_eq!(filename, "variants.ts");
    assert_eq!(contents, include_str!("./variants.ts"));
}
