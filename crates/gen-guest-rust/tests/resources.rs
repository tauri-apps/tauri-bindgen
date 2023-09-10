#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod resources {
    use ::tauri_bindgen_guest_rust::serde;
    use ::tauri_bindgen_guest_rust::bitflags;
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct A(u32);
    impl A {
        pub async fn f1(&self) {
            ::tauri_bindgen_guest_rust::invoke("resources;::resource::A", "f1", &())
                .await
                .unwrap()
        }
        pub async fn f2(&self, a: u32) {
            ::tauri_bindgen_guest_rust::invoke("resources;::resource::A", "f2", &(a))
                .await
                .unwrap()
        }
        pub async fn f3(&self, a: u32, b: u32) {
            ::tauri_bindgen_guest_rust::invoke("resources;::resource::A", "f3", &(a, b))
                .await
                .unwrap()
        }
    }
    #[derive(serde::Deserialize)]
    pub struct B(u32);
    impl B {
        pub async fn f1(&self) -> A {
            ::tauri_bindgen_guest_rust::invoke("resources;::resource::B", "f1", &())
                .await
                .unwrap()
        }
        pub async fn f2(&self, x: A) -> Result<u32, ()> {
            ::tauri_bindgen_guest_rust::invoke("resources;::resource::B", "f2", &(x))
                .await
                .unwrap()
        }
        pub async fn f3(&self, x: Option<&'_ [A]>) -> Result<A, ()> {
            ::tauri_bindgen_guest_rust::invoke("resources;::resource::B", "f3", &(x))
                .await
                .unwrap()
        }
    }
    pub async fn constructor_a() -> A {
        ::tauri_bindgen_guest_rust::invoke("resources", "constructor_a", &())
            .await
            .unwrap()
    }
    pub async fn constructor_b() -> B {
        ::tauri_bindgen_guest_rust::invoke("resources", "constructor_b", &())
            .await
            .unwrap()
    }
}
