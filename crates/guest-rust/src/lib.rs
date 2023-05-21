pub use tauri_bindgen_guest_rust_macro::*;
#[doc(hidden)]
pub use {bitflags, serde, tracing};

use js_sys::Uint8Array;
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("todo")]
    Postcard(#[from] postcard::Error),
    #[error("todo")]
    JsError(JsValue),
    #[error("todo")]
    NoWindow,
}

/// # Errors
///
/// Everything here is fallible (TODO improve this)
pub async fn invoke<P, R>(module: &str, method: &str, val: &P) -> Result<R, Error>
where
    P: Serialize,
    R: DeserializeOwned,
{
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);

    let bytes = postcard::to_allocvec(val)?;
    let body = unsafe { Uint8Array::view(&bytes) };
    opts.body(Some(&body));

    let url = format!("ipc://localhost/{module}/{method}");

    let request = Request::new_with_str_and_init(&url, &opts).map_err(Error::JsError)?;

    request
        .headers()
        .set("Accept", "application/octet-stream")
        .map_err(Error::JsError)?;

    let window = web_sys::window().ok_or(Error::NoWindow)?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(Error::JsError)?;

    // `resp_value` is a `Response` object.
    // assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().map_err(Error::JsError)?;

    let body = JsFuture::from(resp.array_buffer().map_err(Error::JsError)?)
        .await
        .map_err(Error::JsError)?;
    let body = Uint8Array::new(&body);

    Ok(postcard::from_bytes(&body.to_vec())?)
}
