#![allow(unused)]

use tauri_bindgen_abi_derive::Readable;

#[test]
fn derive_enum_unit() {
    #[derive(Debug, Readable)]
    enum U1 {
        A,
        B,
    }
}

#[test]
fn derive_enum_unnamed() {
    #[derive(Debug, Readable)]
    enum U1 {
        A(u64),
        B(f32),
    }
}

#[test]
fn derive_enum_named() {
    #[derive(Debug, Readable)]
    enum U1 {
        A {
            foo: String
        },
        B {
            bar: u128
        },
    }
}

#[test]
fn derive_struct() {
    #[derive(Debug, Readable)]
    struct U1 {
        foo: String,
        bar: u128
    }
}

#[test]
fn derive_bitflags() {
    bitflags::bitflags! {
      #[derive(Readable)]
      #[abi(flags)]
      pub struct Flag4: u8 {
        const B0 = 1 << 0;
        const B1 = 1 << 1;
        const B2 = 1 << 2;
        const B3 = 1 << 3;
      }
    }
}
