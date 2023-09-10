#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

pub use anyhow::Error;
use anyhow::{bail, Result};
use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};
use tauri::{
    http::{header::CONTENT_TYPE, Request, Response, StatusCode},
    AppHandle, Runtime,
};
use tokio::sync::{
    mpsc,
    oneshot::{self, Sender},
};

#[derive(Default)]
pub struct Router<T, R: Runtime> {
    data: Arc<T>,
    string2idx: HashMap<Arc<str>, usize>,
    strings: Vec<Arc<str>>,
    map: HashMap<ImportKey, Definition<T, R>>,
}

pub type Definition<T, R> =
    Box<dyn Fn(Caller<T, R>, &[u8], Sender<Vec<u8>>) -> anyhow::Result<()> + Send + Sync>;

pub struct Caller<'a, T, R: Runtime> {
    data: &'a T,
    app_handle: AppHandle<R>,
}

impl<'a, T, R: Runtime> Caller<'a, T, R> {
    #[must_use]
    pub fn data(&self) -> &T {
        self.data
    }
    #[must_use]
    pub fn app_handle(&self) -> &AppHandle<R> {
        &self.app_handle
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct ImportKey {
    module: usize,
    name: usize,
}

impl<U, R: Runtime> Router<U, R> {
    #[must_use]
    pub fn new(data: U) -> Self {
        Self {
            string2idx: HashMap::new(),
            strings: Vec::new(),
            map: HashMap::new(),
            data: Arc::new(data),
        }
    }

    pub fn func_wrap<Params, Args>(
        &mut self,
        module: &str,
        name: &str,
        func: impl IntoFunc<U, Params, Args, R>,
    ) -> Result<&mut Self> {
        let func = func.into_func();

        let key = self.import_key(Some(module), name);
        self.insert(key, func)?;

        Ok(self)
    }

    pub fn handle_request(
        &mut self,
        module: Option<impl AsRef<str>>,
        name: impl AsRef<str>,
        params: &[u8],
        app_handle: AppHandle<R>,
        tx: Sender<Vec<u8>>,
    ) -> anyhow::Result<()> {
        let key = self.import_key(module, name.as_ref());

        let handler = self
            .map
            .get(&key)
            .ok_or(anyhow::anyhow!("method not found"))?;

        let caller = Caller {
            data: &*self.data,
            app_handle,
        };

        handler(caller, params, tx)?;

        Ok(())
    }

    fn insert(&mut self, key: ImportKey, item: Definition<U, R>) -> Result<()> {
        match self.map.entry(key) {
            Entry::Occupied(_) => {
                let module = &self.strings[key.module];
                let desc = match self.strings.get(key.name) {
                    Some(name) => format!("{module}::{name}"),
                    None => module.to_string(),
                };
                bail!("import of `{}` defined twice", desc)
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

pub trait IntoFunc<U, Params, Results, R: Runtime>: Send + Sync {
    #[doc(hidden)]
    fn into_func(self) -> Definition<U, R>;
}

macro_rules! impl_into_func {
    ($num:tt $($params:ident)*) => {
        // Implement for functions without a leading `&Caller` parameter,
        // delegating to the implementation below which does have the leading
        // `Caller` parameter.
        #[allow(non_snake_case)]
        impl<T, F, $($params,)* R, Rt: Runtime> IntoFunc<T, ($($params,)*), R, Rt> for F
        where
            F: Fn($($params),*) -> anyhow::Result<R> + Send + Sync + 'static,
            $($params: serde::de::DeserializeOwned,)*
            R: serde::Serialize,
        {
            fn into_func(self) -> Definition<T, Rt> {
                let f = move |_: Caller<T, Rt>, $($params:$params),*| {
                    self($($params),*)
                };

                f.into_func()
            }
        }

        #[allow(non_snake_case)]
        impl<T, F, $($params,)* R, Rt: Runtime> IntoFunc<T, (Caller<'_, T, Rt>, $($params,)*), R, Rt> for F
        where
            F: Fn(Caller<T, Rt>, $($params),*) -> anyhow::Result<R> + Send + Sync + 'static,
            $($params: serde::de::DeserializeOwned,)*
            R: serde::Serialize,
        {
            fn into_func(self) -> Definition<T, Rt> {
                Box::new(move |caller, params, tx| {
                    log::debug!("Deserializing parameters...");
                    let ($($params,)*) = postcard::from_bytes(params)?;

                    log::debug!("Calling handler...");
                    let out = self(caller, $($params),*)?;

                    log::debug!("Serializing response...");
                    let out = postcard::to_allocvec(&out)?;

                    tx.send(out).unwrap();

                    Ok(())
                })
            }
        }
    }
}

macro_rules! for_each_function_signature {
    ($mac:ident) => {
        $mac!(0);
        $mac!(1 A1);
        $mac!(2 A1 A2);
        $mac!(3 A1 A2 A3);
        $mac!(4 A1 A2 A3 A4);
        $mac!(5 A1 A2 A3 A4 A5);
        $mac!(6 A1 A2 A3 A4 A5 A6);
        $mac!(7 A1 A2 A3 A4 A5 A6 A7);
        $mac!(8 A1 A2 A3 A4 A5 A6 A7 A8);
        $mac!(9 A1 A2 A3 A4 A5 A6 A7 A8 A9);
        $mac!(10 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10);
        $mac!(11 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11);
        $mac!(12 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12);
        $mac!(13 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13);
        $mac!(14 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 A14);
        $mac!(15 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 A14 A15);
        $mac!(16 A1 A2 A3 A4 A5 A6 A7 A8 A9 A10 A11 A12 A13 A14 A15 A16);
    };
}

for_each_function_signature!(impl_into_func);

pub trait BuilderExt<R: Runtime> {
    #[must_use]
    fn ipc_router<U: Send + Sync + 'static>(self, router: Router<U, R>) -> Self;
}

impl<R: Runtime> BuilderExt<R> for tauri::Builder<R> {
    fn ipc_router<U: Send + Sync + 'static>(self, mut router: Router<U, R>) -> Self {
        let (tx, mut rx) = mpsc::channel::<(AppHandle<R>, Request<Vec<u8>>, _)>(250);

        let this =
            self.register_asynchronous_uri_scheme_protocol("ipc", move |app, req, responder| {
                tx.try_send((app.clone(), req, responder)).unwrap();
            });

        tauri::async_runtime::handle().spawn(async move {
            while let Some((app, req, responder)) = rx.recv().await {
                let path = req.uri().path().strip_prefix('/').unwrap();

                let (module, method) = path
                    .split_once('/')
                    .map_or((None, path), |(module, method)| (Some(module), method));

                log::debug!("ipc request for {:?}::{}", module, method);

                let (tx, rx) = oneshot::channel();
                router
                    .handle_request(module, method, req.body(), app, tx)
                    .unwrap();
                let res = rx.await;

                log::debug!("call result {:?}", res);

                let mut response = match res {
                    Ok(val) => {
                        let mut resp = Response::builder().status(StatusCode::OK);
                        resp.headers_mut().unwrap().insert(
                            CONTENT_TYPE,
                            tauri::http::header::HeaderValue::from_static(
                                "application/octet-stream",
                            ),
                        );
                        resp.body(val).unwrap()
                    }
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
            }
        });

        this
    }
}

// async fn uri_scheme_handler_inner<U: Send + Sync + 'static, R: Runtime>(
//     app: &AppHandle<R>,
//     req: &tauri::http::Request<Vec<u8>>,
// ) -> anyhow::Result<Vec<u8>> {
//     let path = req.uri().path().strip_prefix('/').unwrap();

//     let (module, method) = path
//         .split_once('/')
//         .map_or((None, path), |(module, method)| (Some(module), method));

//     log::debug!("ipc request for {:?}::{}", module, method);

//     let (res_tx, res_rx) = oneshot::channel();

//     let router = app.state::<Mutex<Router<U>>>();
//     let mut router = router.lock().unwrap();

//     // this is terrible we should not clone here
//     router.handle_request(module, method, req.body(), res_tx)?;

//     Ok(res_rx.await?)
// }
