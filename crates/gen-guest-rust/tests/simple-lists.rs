#[allow(clippy::all, unused)]
pub mod imports {
    pub async fn simple_list1(l: &[u32]) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            l: &'a [u32],
        }
        let params = Params { l };
        ::tauri_bindgen_guest_rust::send("plugin:imports|simple_list1", &params).await
    }
    pub async fn simple_list2() -> Vec<u32> {
        ::tauri_bindgen_guest_rust::send("plugin:imports|simple_list2", ()).await
    }
    pub async fn simple_list4(l: &[&[u32]]) -> Vec<Vec<u32>> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            l: &'a [&'a [u32]],
        }
        let params = Params { l };
        ::tauri_bindgen_guest_rust::send("plugin:imports|simple_list4", &params).await
    }
}
