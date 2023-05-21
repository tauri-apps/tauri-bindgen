pub use tauri_bindgen_guest_rust_macro::*;
#[doc(hidden)]
pub use {bitflags, serde, tracing};

use js_sys::Uint8Array;
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

pub async fn invoke<P, R>(module: &str, method: &str, val: &P) -> Result<R, JsValue>
where
    P: Serialize,
    R: DeserializeOwned,
{
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);

    let bytes = postcard::to_allocvec(val).unwrap();
    let body = unsafe { Uint8Array::view(&bytes) };
    opts.body(Some(&body));

    let url = format!("ipc://localhost/{module}/{method}");

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request.headers().set("Accept", "application/octet-stream")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let body = JsFuture::from(resp.array_buffer()?).await?;
    let body = Uint8Array::new(&body);

    Ok(postcard::from_bytes(&body.to_vec()).unwrap())
}
