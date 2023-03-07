pub use anyhow::Error;
use anyhow::{bail, Result};
use std::{
    collections::{hash_map::Entry, HashMap},
    marker::PhantomData,
    sync::{mpsc::Sender, Arc},
};

pub struct Router<U> {
    string2idx: HashMap<Arc<str>, usize>,
    strings: Vec<Arc<str>>,
    map: HashMap<ImportKey, Definition<U>>,
}

pub type Definition<U> = Box<dyn Fn(Caller<U>, &[u8], Sender<Vec<u8>>)>;

pub struct Caller<T> {
    _m: PhantomData<T>,
}

impl<T> Caller<T> {
    pub fn data_mut(&self) -> &mut T {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct ImportKey {
    module: usize,
    name: usize,
}

impl<U> Router<U> {
    pub fn new() -> Self {
        Self {
            string2idx: HashMap::new(),
            strings: Vec::new(),
            map: HashMap::new(),
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
        params: impl AsRef<[u8]>,
        res_tx: Sender<Vec<u8>>,
    ) -> anyhow::Result<()> {
        let key = self.import_key(module, name.as_ref());

        let handler = self
            .map
            .get(&key)
            .ok_or(anyhow::anyhow!("method not found"))?;

        let caller = Caller { _m: PhantomData };

        handler(caller, params.as_ref(), res_tx);

        Ok(())
    }

    fn insert(&mut self, key: ImportKey, item: Definition<U>) -> Result<()> {
        match self.map.entry(key) {
            Entry::Occupied(_) => {
                let module = &self.strings[key.module];
                let desc = match self.strings.get(key.name) {
                    Some(name) => format!("{}::{}", module, name),
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
            module: module
                .map(|name| self.intern_str(name.as_ref()))
                .unwrap_or(usize::max_value()),
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

pub trait IntoFunc<U, Params, Results>: Send + Sync + 'static {
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
            F: Fn($($params),*) -> R + Send + Sync + 'static,
            $($params: tauri_bindgen_abi::Readable,)*
            R: tauri_bindgen_abi::Writable
        {
            fn into_func(self) -> Definition<T> {
                let f = move |_: Caller<T>, $($params:$params),*| {
                    self($($params),*)
                };

                f.into_func()
            }
        }

        #[allow(non_snake_case)]
        impl<T, F, $($params,)* R> IntoFunc<T, (Caller<T>, $($params,)*), R> for F
        where
            F: Fn(Caller<T>, $($params),*) -> R + Send + Sync + 'static,
            $($params: tauri_bindgen_abi::Readable,)*
            R: tauri_bindgen_abi::Writable
        {
            fn into_func(self) -> Definition<T> {
                Box::new(move |caller, params, tx| {
                    log::debug!("Deserializing parameters...");
                    let ($($params,)*) = tauri_bindgen_abi::from_slice(params).unwrap();

                    log::debug!("Calling handler...");
                    let out = self(caller, $($params),*);

                    log::debug!("Serializing response...");
                    let out = tauri_bindgen_abi::to_bytes(&out).unwrap();

                    tx.send(out).unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feature() {
        let mut router: Router<()> = Router::new();
        router
            .func_wrap("app", "add", |a: u32, b: u32| -> Result<u32, ()> {
                Ok(a + b)
            })
            .unwrap();

        let (tx, rx) = std::sync::mpsc::channel();

        router
            .handle_request(
                Some("app"),
                "add",
                tauri_bindgen_abi::to_bytes(&(5u32, 7u32)).unwrap(),
                tx,
            )
            .unwrap();

        println!("{:?}", rx.recv());
    }
}
