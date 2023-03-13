#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod resources {
    use ::tauri_bindgen_guest_rust::serde;
    use ::tauri_bindgen_guest_rust::bitflags;
    #[derive(serde::Deserialize)]
    struct A {
        id: u64,
    }
    impl A {
        pub async fn f1(&self) {
            todo!()
        }
        pub async fn f2(&self, a: u32) {
            todo!()
        }
        pub async fn f3(&self, a: u32, b: u32) {
            todo!()
        }
    }
    #[derive(serde::Deserialize)]
    struct B {
        id: u64,
    }
    impl B {
        pub async fn f1(&self) -> A {
            todo!()
        }
        pub async fn f2(&self, x: A) -> Result<u32, ()> {
            todo!()
        }
        pub async fn f3(&self, x: Option<&'_ [A]>) -> Result<A, ()> {
            todo!()
        }
    }
    pub async fn constructor_a() -> A {
        todo!()
    }
    pub async fn constructor_b() -> B {
        todo!()
    }
}
