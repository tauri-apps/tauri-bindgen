#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

pub use anyhow::Error;

use futures_util::FutureExt;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    collections::{hash_map::Entry, HashMap},
    future::Future,
    marker::PhantomData,
    pin::Pin,
    sync::Arc,
};
use tauri::http::{header::CONTENT_TYPE, Request, Response, StatusCode};

type Definition<T> =
    Box<dyn Fn(Caller<T>, &[u8]) -> anyhow::Result<CallResult> + Send + Sync + 'static>;

enum CallResult {
    Value(Vec<u8>),
    Future(Pin<Box<dyn Future<Output = anyhow::Result<Vec<u8>>> + Send + 'static>>),
}

pub struct Caller<T> {
    data: Arc<T>,
}

impl<T> Caller<T> {
    #[must_use]
    pub fn data(&self) -> &T {
        &self.data
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct ImportKey {
    module: usize,
    name: usize,
}

pub struct Router<T> {
    data: Arc<T>,
    _m: PhantomData<T>,
    string2idx: HashMap<Arc<str>, usize>,
    strings: Vec<Arc<str>>,
    map: HashMap<ImportKey, Definition<T>>,
}

impl<T> Router<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Arc::new(data),
            _m: PhantomData,
            string2idx: HashMap::new(),
            strings: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn define<F, P, R>(&mut self, module: &str, name: &str, func: F) -> anyhow::Result<()>
    where
        F: Fn(Caller<T>, P) -> anyhow::Result<R> + Send + Sync + 'static,
        P: DeserializeOwned,
        R: Serialize,
    {
        let key = self.import_key(Some(module), name);

        self.insert(
            key,
            Box::new(move |caller, params| {
                let params = postcard::from_bytes(params)?;

                let res = func(caller, params)?;

                Ok(CallResult::Value(postcard::to_allocvec(&res).unwrap()))
            }),
        )?;

        Ok(())
    }

    pub fn define_async<F, P, R, RV>(
        &mut self,
        module: &str,
        name: &str,
        func: F,
    ) -> anyhow::Result<()>
    where
        F: Fn(Caller<T>, P) -> Pin<Box<R>> + Send + Sync + 'static,
        P: DeserializeOwned,
        R: Future<Output = anyhow::Result<RV>> + Send + 'static,
        RV: Serialize,
    {
        let key = self.import_key(Some(module), name);

        self.insert(
            key,
            Box::new(move |caller, params| {
                let params = postcard::from_bytes(params)?;

                let fut = func(caller, params)
                    .map(|res| postcard::to_allocvec(&res?).map_err(Into::into))
                    .boxed();

                Ok(CallResult::Future(fut))
            }),
        )?;

        Ok(())
    }

    async fn call(
        &self,
        module: Option<&str>,
        name: &str,
        params: &[u8],
    ) -> anyhow::Result<Vec<u8>> {
        let key = self.import_key_read_only(module, name)?;

        let handler = self
            .map
            .get(&key)
            .ok_or(anyhow::anyhow!("method not found"))?;

        let caller = Caller {
            data: self.data.clone(),
        };

        match handler(caller, params)? {
            CallResult::Value(val) => Ok(val),
            CallResult::Future(fut) => Ok(fut.await?),
        }
    }

    fn insert(&mut self, key: ImportKey, item: Definition<T>) -> anyhow::Result<()> {
        match self.map.entry(key) {
            Entry::Occupied(_) => {
                let module = &self.strings[key.module];
                let desc = match self.strings.get(key.name) {
                    Some(name) => format!("{module}::{name}"),
                    None => module.to_string(),
                };
                anyhow::bail!("import of `{}` defined twice", desc)
            }
            Entry::Vacant(v) => {
                v.insert(item);
            }
        }
        Ok(())
    }

    fn import_key(&mut self, module: Option<impl AsRef<str>>, name: impl AsRef<str>) -> ImportKey {
        ImportKey {
            module: module.map_or(usize::max_value(), |name| self.intern_str(name.as_ref())),
            name: self.intern_str(name.as_ref()),
        }
    }

    fn import_key_read_only(&self, module: Option<&str>, name: &str) -> anyhow::Result<ImportKey> {
        let module = if let Some(module) = module {
            *self
                .string2idx
                .get(module)
                .ok_or(anyhow::anyhow!("unknown module"))?
        } else {
            usize::MAX
        };

        let name = *self
            .string2idx
            .get(name)
            .ok_or(anyhow::anyhow!("unknown function"))?;

        Ok(ImportKey { module, name })
    }

    fn intern_str(&mut self, string: &str) -> usize {
        if let Some(idx) = self.string2idx.get(string) {
            return *idx;
        }
        let string: Arc<str> = string.into();
        let idx = self.strings.len();
        self.strings.push(string.clone());
        self.string2idx.insert(string, idx);
        idx
    }
}

pub trait BuilderExt {
    #[must_use]
    fn ipc_router<U: Send + Sync + 'static>(self, router: Router<U>) -> Self;
}

impl<R: tauri::Runtime> BuilderExt for tauri::Builder<R> {
    fn ipc_router<U: Send + Sync + 'static>(self, router: Router<U>) -> Self {
        let router = Arc::new(router);

        self.register_asynchronous_uri_scheme_protocol("ipc", move |_app, req, responder| {
            let router = router.clone();

            tauri::async_runtime::spawn(async move {
                let mut response = match uri_scheme_inner(&router, req).await {
                    Ok(res) => res,
                    Err(err) => Response::builder()
                        .status(StatusCode::BAD_REQUEST)
                        .body(err.to_string().into_bytes())
                        .unwrap(),
                };

                response.headers_mut().insert(
                    tauri::http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                    tauri::http::header::HeaderValue::from_static("*"),
                );

                responder.respond(response);
            });
        })
    }
}

#[inline]
async fn uri_scheme_inner<T>(
    router: &Router<T>,
    request: Request<Vec<u8>>,
) -> anyhow::Result<Response<Vec<u8>>> {
    let path = request.uri().path().strip_prefix('/').unwrap();

    let (module, method) = path
        .split_once('/')
        .map_or((None, path), |(module, method)| (Some(module), method));

    log::debug!("ipc request for {:?}::{}", module, method);

    let response = router.call(module, method, request.body()).await?;

    log::debug!("call result {:?}", response);

    let mut resp = Response::builder().status(StatusCode::OK);
    resp.headers_mut().unwrap().insert(
        CONTENT_TYPE,
        tauri::http::header::HeaderValue::from_static("application/octet-stream"),
    );

    Ok(resp.body(response)?)
}
