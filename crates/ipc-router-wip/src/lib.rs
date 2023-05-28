#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

pub use anyhow::Error;
use anyhow::{bail, Result};
use std::{
    collections::{hash_map::Entry, HashMap},
    sync::{mpsc::Sender, Arc, Mutex},
};
use tauri::{AppHandle, Manager, Runtime};
use url::Url;

#[derive(Default)]
pub struct Router<T> {
    data: Arc<T>,
    string2idx: HashMap<Arc<str>, usize>,
    strings: Vec<Arc<str>>,
    map: HashMap<ImportKey, Definition<T>>,
}

pub type Definition<T> =
    Box<dyn Fn(Caller<T>, &[u8], Sender<Vec<u8>>) -> anyhow::Result<()> + Send + Sync>;

pub struct Caller<'a, T> {
    data: &'a T,
}

impl<'a, T> Caller<'a, T> {
    #[must_use]
    pub fn data(&self) -> &T {
        self.data
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct ImportKey {
    module: usize,
    name: usize,
}

impl<U> Router<U> {
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
        func: impl IntoFunc<U, Params, Args>,
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
        res_tx: Sender<Vec<u8>>,
    ) -> anyhow::Result<()> {
        let key = self.import_key(module, name.as_ref());

        let handler = self
            .map
            .get(&key)
            .ok_or(anyhow::anyhow!("method not found"))?;

        let caller = Caller { data: &*self.data };

        handler(caller, params, res_tx)?;

        Ok(())
    }

    fn insert(&mut self, key: ImportKey, item: Definition<U>) -> Result<()> {
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

pub trait IntoFunc<U, Params, Results>: Send + Sync {
    #[doc(hidden)]
    fn into_func(self) -> Definition<U>;
}

macro_rules! impl_into_func {
    ($num:tt $($params:ident)*) => {
        // Implement for functions without a leading `&Caller` parameter,
        // delegating to the implementation below which does have the leading
        // `Caller` parameter.
        #[allow(non_snake_case)]
        impl<T, F, $($params,)* R> IntoFunc<T, ($($params,)*), R> for F
        where
            F: Fn($($params),*) -> anyhow::Result<R> + Send + Sync + 'static,
            $($params: serde::de::DeserializeOwned,)*
            R: serde::Serialize
        {
            fn into_func(self) -> Definition<T> {
                let f = move |_: Caller<T>, $($params:$params),*| {
                    self($($params),*)
                };

                f.into_func()
            }
        }

        #[allow(non_snake_case)]
        impl<T, F, $($params,)* R> IntoFunc<T, (Caller<'_, T>, $($params,)*), R> for F
        where
            F: Fn(Caller<T>, $($params),*) -> anyhow::Result<R> + Send + Sync + 'static,
            $($params: serde::de::DeserializeOwned,)*
            R: serde::Serialize
        {
            fn into_func(self) -> Definition<T> {
                Box::new(move |caller, params, tx| {
                    log::debug!("Deserializing parameters...");
                    let ($($params,)*) = postcard::from_bytes(params)?;

                    log::debug!("Calling handler...");
                    let out = self(caller, $($params),*)?;

                    log::debug!("Serializing response...");
                    let out = postcard::to_allocvec(&out)?;

                    tx.send(out)?;

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

pub trait BuilderExt {
    #[must_use]
    fn ipc_router<U: Send + Sync + 'static>(self, router: Router<U>) -> Self;
}

impl<R: Runtime> BuilderExt for tauri::Builder<R> {
    fn ipc_router<U: Send + Sync + 'static>(self, router: Router<U>) -> Self {
        self.manage(Mutex::new(router))
            .register_uri_scheme_protocol("ipc", |app, req| {
                let res = uri_scheme_handler_inner::<U, _>(app, req);

                log::debug!("call result {:?}", res);

                let mut resp = match res {
                    Ok(val) => {
                        let mut resp = tauri::http::Response::new(val);
                        resp.set_status(tauri::http::status::StatusCode::OK);
                        resp.set_mimetype(Some("application/octet-stream".to_string()));
                        resp
                    }
                    Err(err) => {
                        let mut resp = tauri::http::Response::new(err.to_string().into_bytes());
                        resp.set_status(tauri::http::status::StatusCode::BAD_REQUEST);
                        resp
                    }
                };

                resp.headers_mut().insert(
                    tauri::http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
                    tauri::http::header::HeaderValue::from_static("*"),
                );

                log::trace!("sending response {:?}", resp);

                Ok(resp)
            })
    }
}

fn uri_scheme_handler_inner<U: Send + Sync + 'static, R: Runtime>(
    app: &AppHandle<R>,
    req: &tauri::http::Request,
) -> anyhow::Result<Vec<u8>> {
    let url = Url::parse(req.uri())?;

    let path = url.path().strip_prefix('/').unwrap();

    let (module, method) = path
        .split_once('/')
        .map_or((None, path), |(module, method)| (Some(module), method));

    log::debug!("ipc request for {:?}::{}", module, method);

    let (res_tx, res_rx) = std::sync::mpsc::channel();

    let router = app.state::<Mutex<Router<U>>>();
    let mut router = router.lock().unwrap();

    // this is terrible we should not clone here
    router.handle_request(module, method, req.body(), res_tx)?;

    Ok(res_rx.recv()?)
}
