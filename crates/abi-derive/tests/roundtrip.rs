use tauri_bindgen_abi::{Readable, Writable};
use tauri_bindgen_abi_derive::{Readable, Writable};

#[test]
fn struct_() {
    #[derive(Debug, PartialEq, Readable, Writable)]
    struct Foo {
        a: u8,
        b: u64,
        c: String,
    }

    let input = Foo {
        a: 3,
        b: 16,
        c: "foo".to_string(),
    };

    let mut bytes: Vec<u8> = vec![];
    Writable::write_to(&input, &mut bytes).unwrap();

    assert_eq!(
        bytes,
        vec![3, 16, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 102, 111, 111]
    );

    let output: Foo = Readable::read_from(&mut bytes.as_slice()).unwrap();

    assert_eq!(output, input);
}

#[test]
fn enum_unit() {
    #[derive(Debug, PartialEq, Readable, Writable)]
    enum U1 {
        A,
        B,
    }

    let input = U1::A;

    let mut bytes: Vec<u8> = vec![];
    Writable::write_to(&input, &mut bytes).unwrap();

    assert_eq!(bytes, vec![0]);

    let output: U1 = Readable::read_from(&mut bytes.as_slice()).unwrap();

    assert_eq!(output, input);
}

#[test]
fn enum_unnamed() {
    #[derive(Debug, PartialEq, Readable, Writable)]
    enum U1 {
        A(u64),
        B(f32),
    }

    let input = U1::A(50);

    let mut bytes: Vec<u8> = vec![];
    Writable::write_to(&input, &mut bytes).unwrap();

    assert_eq!(bytes, vec![0, 50, 0, 0, 0, 0, 0, 0, 0]);

    let output: U1 = Readable::read_from(&mut bytes.as_slice()).unwrap();

    assert_eq!(output, input);
}

#[test]
fn enum_named() {
    #[derive(Debug, PartialEq, Readable, Writable)]
    enum U1 {
        A { foo: String },
        B { bar: u128 },
    }

    let input = U1::B { bar: 60 };

    let mut bytes: Vec<u8> = vec![];
    Writable::write_to(&input, &mut bytes).unwrap();

    assert_eq!(
        bytes,
        vec![1, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );

    let output: U1 = Readable::read_from(&mut bytes.as_slice()).unwrap();

    assert_eq!(output, input);
}

#[test]
fn flags() {
    bitflags::bitflags! {
      #[derive(Readable, Writable)]
      #[abi(flags)]
      pub struct Flag4: u8 {
        const B0 = 1 << 0;
        const B1 = 1 << 1;
        const B2 = 1 << 2;
        const B3 = 1 << 3;
      }
    }

    let input = Flag4::B0 | Flag4::B2;

    let mut bytes: Vec<u8> = vec![];
    Writable::write_to(&input, &mut bytes).unwrap();

    assert_eq!(bytes, vec![0b0000_0101]);

    let output: Flag4 = Readable::read_from(&mut bytes.as_slice()).unwrap();

    assert_eq!(output, input);
}
