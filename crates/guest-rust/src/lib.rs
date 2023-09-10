pub use tauri_bindgen_guest_rust_macro::*;
#[doc(hidden)]
pub use {bitflags, serde, tracing};

use futures::{ready, Future, Stream};
use js_sys::Uint8Array;
use pin_project_lite::pin_project;
use serde::{de::DeserializeOwned, Serialize};
use std::{marker::PhantomData, task::Poll};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{ReadableStreamDefaultReader, Request, RequestInit, RequestMode, Response};

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
///
/// # Panics
///
/// Panics when the response returned by JavaScript is not a `ResponseObject`
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
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().map_err(Error::JsError)?;

    let body = JsFuture::from(resp.array_buffer().map_err(Error::JsError)?)
        .await
        .map_err(Error::JsError)?;
    let body = Uint8Array::new(&body);

    Ok(postcard::from_bytes(&body.to_vec())?)
}

pub async fn open_stream<P, R>(module: &str, method: &str, val: &P) -> Result<Streaming<R>, Error>
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
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().map_err(Error::JsError)?;

    let readable_stream = resp.body().unwrap();
    let stream_reader = readable_stream.get_reader();

    assert!(stream_reader.is_instance_of::<ReadableStreamDefaultReader>());
    let stream_reader: ReadableStreamDefaultReader = stream_reader.dyn_into().unwrap();

    Ok(Streaming {
        future: JsFuture::from(stream_reader.read()),
        stream_reader,
        _m: PhantomData,
    })
}

pin_project! {
    pub struct Streaming<T> {
        #[pin]
        future: JsFuture,
        stream_reader: ReadableStreamDefaultReader,
        _m: PhantomData<T>,
    }
}

impl<T: DeserializeOwned> Stream for Streaming<T> {
    type Item = Result<T, Error>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let mut this = self.as_mut().project();

        let v = ready!(this.future.as_mut().poll(cx)).unwrap();

        let done = js_sys::Reflect::get(&v, &JsValue::from_str("done")).map_err(Error::JsError)?;
        if done.is_truthy() {
            return Poll::Ready(None);
        } else {
            let fut = JsFuture::from(this.stream_reader.read());
            this.future.set(fut);
        }

        let value = {
            let raw =
                js_sys::Reflect::get(&v, &JsValue::from_str("value")).map_err(Error::JsError)?;

            tracing::debug!("raw vaule from reader {raw:?}");

            let raw = Uint8Array::new(&raw);
            postcard::from_bytes(&raw.to_vec())?
        };

        return Poll::Ready(Some(Ok(value)));
    }
}
