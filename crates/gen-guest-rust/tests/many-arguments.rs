#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod many_arguments {
    use ::tauri_bindgen_guest_rust::serde;
    use ::tauri_bindgen_guest_rust::bitflags;
    #[derive(serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct BigStruct<'a> {
        pub a1: &'a str,
        pub a2: &'a str,
        pub a3: &'a str,
        pub a4: &'a str,
        pub a5: &'a str,
        pub a6: &'a str,
        pub a7: &'a str,
        pub a8: &'a str,
        pub a9: &'a str,
        pub a10: &'a str,
        pub a11: &'a str,
        pub a12: &'a str,
        pub a13: &'a str,
        pub a14: &'a str,
        pub a15: &'a str,
        pub a16: &'a str,
        pub a17: &'a str,
        pub a18: &'a str,
        pub a19: &'a str,
        pub a20: &'a str,
    }
    pub async fn many_args(
        a1: u64,
        a2: u64,
        a3: u64,
        a4: u64,
        a5: u64,
        a6: u64,
        a7: u64,
        a8: u64,
        a9: u64,
        a10: u64,
        a11: u64,
        a12: u64,
        a13: u64,
        a14: u64,
        a15: u64,
        a16: u64,
    ) {
        ::tauri_bindgen_guest_rust::invoke(
                "many_arguments",
                "many_args",
                &(a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16),
            )
            .await
            .unwrap()
    }
    pub async fn big_argument(x: BigStruct<'_>) {
        ::tauri_bindgen_guest_rust::invoke("many_arguments", "big_argument", &(x))
            .await
            .unwrap()
    }
}
