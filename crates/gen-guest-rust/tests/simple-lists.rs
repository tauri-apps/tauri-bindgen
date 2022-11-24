#[allow(clippy::all, unused)]
pub mod imports {
    const IDL_HASH: &str = "d40a3203ef48115d";
    pub async fn simple_list1(l: &[u32]) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            l: &'a [u32],
        }
        let params = Params {
            idl_hash: IDL_HASH,
            l,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:simple_lists|simple-list1", &params).await
    }
    pub async fn simple_list2() -> Vec<u32> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:simple_lists|simple-list2", ()).await
    }
    pub async fn simple_list4(l: &[&[u32]]) -> Vec<Vec<u32>> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            l: &'a [&'a [u32]],
        }
        let params = Params {
            idl_hash: IDL_HASH,
            l,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:simple_lists|simple-list4", &params).await
    }
}
