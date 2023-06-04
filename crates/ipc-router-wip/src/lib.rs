#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

pub use anyhow::Error;
use anyhow::{bail, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
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

    pub fn func_wrap<'de, Params, Results>(
        &mut self,
        module: &str,
        name: &str,
        func: impl IntoFunc<U, Params, Results>,
    ) -> Result<&mut Self> {
        let _js = func.generate_js();
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
    #[doc(hidden)]
    fn generate_js(&self) -> String;
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

            fn generate_js(&self) -> String {
                serde_js::generate::<($($params,)*), R>()
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

            fn generate_js(&self) -> String {
                serde_js::generate::<($($params,)*), R>()
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn t() {
        let js = serde_js::generate::<(), Vec<u32>>();

        println!("{:?}", js);
    }
}

mod serde_js {
    use serde::{de::DeserializeOwned, Serialize};
    use std::mem::MaybeUninit;

    pub fn generate<P: DeserializeOwned, R: Serialize>() -> String {
        // let unsafe_p: P = unsafe { MaybeUninit::uninit().assume_init() };
        let unsafe_r: R = unsafe { MaybeUninit::uninit().assume_init() };

        let mut serializer = ser::Serializer {
            output: String::new(),
            std_funcs: ser::StdFuncs::empty(),
        };
        unsafe_r.serialize(&mut serializer).unwrap();

        serializer.output
    }

    mod ser {
        use std::fmt::Write;

        use serde::{ser, Serialize};

        #[derive(Debug, thiserror::Error)]
        pub enum Error {
            #[error(transparent)]
            Fmt(#[from] std::fmt::Error),
            #[error("todo")]
            SerdeSerCustom,
        }

        impl serde::ser::Error for Error {
            fn custom<T>(_msg: T) -> Self
            where
                T: std::fmt::Display,
            {
                Error::SerdeSerCustom
            }
        }

        type Result<T> = std::result::Result<T, Error>;

        pub struct Serializer {
            pub output: String,
            pub std_funcs: StdFuncs,
        }

        bitflags::bitflags! {
            #[derive(Debug, Clone, Copy)]
            pub struct StdFuncs: u32 {
                const VARINT_MAX        = 1 << 1;
                const VARINT            = 1 << 2;

                const BOOl              = 1 << 3;
                const BITS8             = 1 << 4;
                const BITS16            = 1 << 5;
                const BITS32            = 1 << 6;
                const BITS64            = 1 << 7;
                const BITS128           = 1 << 8;
                const SIGNED            = 1 << 9;
                const UNSIGNED          = 1 << 10;

                const F32               = 1 << 12;
                const F64               = 1 << 13;
                const CHAR             = 1 << 14;
                const STRING           = 1 << 15;
                const BYTES            = 1 << 16;
                const OPTION           = 1 << 17;
                const RESULT           = 1 << 18;
                const LIST             = 1 << 19;
                const DE                = 1 << 20;
                const SER               = 1 << 21;
                const STR_UTIL          = 1 << 22;
            }
        }

        impl Serializer {
            fn include_once(
                &mut self,
                funcs: StdFuncs,
                include_funcs: impl FnOnce(&mut Self) -> Result<()>,
            ) -> Result<()> {
                if !self.std_funcs.contains(funcs) {
                    include_funcs(self)?;
                    self.std_funcs.insert(funcs);
                }
                Ok(())
            }

            fn serialize_varint(&mut self) -> Result<()> {
                self.include_once(StdFuncs::VARINT_MAX, |this| {
                    this.output.push_str(include_str!("./js/varint_max.js"));
                    Ok(())
                })?;
                self.include_once(StdFuncs::VARINT | StdFuncs::SER, |this| {
                    this.output.push_str(include_str!("./js/ser_varint.js"));
                    Ok(())
                })?;

                Ok(())
            }

            fn text_decoder(&mut self) -> Result<()> {
                self.include_once(StdFuncs::STR_UTIL, |this| {
                    this.output
                        .push_str("const __text_decoder = new TextDecoder('utf-8');\n");
                    Ok(())
                })
            }
        }

        impl<'a> ser::Serializer for &'a mut Serializer {
            // The output type produced by this `Serializer` during successful
            // serialization. Most serializers that produce text or binary output should
            // set `Ok = ()` and serialize into an `io::Write` or buffer contained
            // within the `Serializer` instance, as happens here. Serializers that build
            // in-memory data structures may be simplified by using `Ok` to propagate
            // the data structure around.
            type Ok = ();

            // The error type when some error occurs during serialization.
            type Error = Error;

            // Associated types for keeping track of additional state while serializing
            // compound data structures like sequences and maps. In this case no
            // additional state is required beyond what is already stored in the
            // Serializer struct.
            type SerializeSeq = Self;
            type SerializeTuple = Self;
            type SerializeTupleStruct = Self;
            type SerializeTupleVariant = Self;
            type SerializeMap = Self;
            type SerializeStruct = Self;
            type SerializeStructVariant = Self;

            // Here we go with the simple methods. The following 12 methods receive one
            // of the primitive types of the data model and map it to JSON by appending
            // into the output string.
            fn serialize_bool(self, _v: bool) -> Result<()> {
                self.include_once(StdFuncs::BOOl, |this| {
                    this.output.push_str(include_str!("./js/ser_bool.js"));
                    Ok(())
                })?;
                Ok(())
            }

            // JSON does not distinguish between different sizes of integers, so all
            // signed integers will be serialized the same and all unsigned integers
            // will be serialized the same. Other formats, especially compact binary
            // formats, may need independent logic for the different sizes.
            fn serialize_i8(self, _v: i8) -> Result<()> {
                self.include_once(StdFuncs::BITS8 | StdFuncs::SIGNED | StdFuncs::SER, |this| {
                    this.output.push_str(include_str!("./js/ser_s8.js"));
                    Ok(())
                })
            }

            fn serialize_i16(self, _v: i16) -> Result<()> {
                self.serialize_varint()?;
                self.include_once(
                    StdFuncs::BITS16 | StdFuncs::SIGNED | StdFuncs::SER,
                    |this| {
                        this.output.push_str(include_str!("./js/ser_s16.js"));
                        Ok(())
                    },
                )
            }

            fn serialize_i32(self, _v: i32) -> Result<()> {
                self.serialize_varint()?;
                self.include_once(
                    StdFuncs::BITS32 | StdFuncs::SIGNED | StdFuncs::SER,
                    |this| {
                        this.output.push_str(include_str!("./js/ser_s32.js"));
                        Ok(())
                    },
                )
            }

            // Not particularly efficient but this is example code anyway. A more
            // performant approach would be to use the `itoa` crate.
            fn serialize_i64(self, _v: i64) -> Result<()> {
                self.serialize_varint()?;
                self.include_once(
                    StdFuncs::BITS64 | StdFuncs::SIGNED | StdFuncs::SER,
                    |this| {
                        this.output.push_str(include_str!("./js/ser_s64.js"));
                        Ok(())
                    },
                )
            }

            fn serialize_u8(self, _v: u8) -> Result<()> {
                self.include_once(
                    StdFuncs::BITS8 | StdFuncs::UNSIGNED | StdFuncs::SER,
                    |this| {
                        this.output.push_str(include_str!("./js/ser_u8.js"));
                        Ok(())
                    },
                )
            }

            fn serialize_u16(self, _v: u16) -> Result<()> {
                self.serialize_varint()?;
                self.include_once(
                    StdFuncs::BITS16 | StdFuncs::UNSIGNED | StdFuncs::SER,
                    |this| {
                        this.output.push_str(include_str!("./js/ser_u16.js"));
                        Ok(())
                    },
                )
            }

            fn serialize_u32(self, _v: u32) -> Result<()> {
                self.serialize_varint()?;
                self.include_once(
                    StdFuncs::BITS32 | StdFuncs::UNSIGNED | StdFuncs::SER,
                    |this| {
                        this.output.push_str(include_str!("./js/ser_u32.js"));
                        Ok(())
                    },
                )
            }

            fn serialize_u64(self, _v: u64) -> Result<()> {
                self.serialize_varint()?;
                self.include_once(
                    StdFuncs::BITS64 | StdFuncs::UNSIGNED | StdFuncs::SER,
                    |this| {
                        this.output.push_str(include_str!("./js/ser_u64.js"));
                        Ok(())
                    },
                )
            }

            fn serialize_f32(self, _v: f32) -> Result<()> {
                self.include_once(StdFuncs::F32 | StdFuncs::SER, |this| {
                    this.output.push_str(include_str!("./js/ser_f32.js"));
                    Ok(())
                })
            }

            fn serialize_f64(self, _v: f64) -> Result<()> {
                self.include_once(StdFuncs::F64 | StdFuncs::SER, |this| {
                    this.output.push_str(include_str!("./js/ser_f64.js"));
                    Ok(())
                })
            }

            // Serialize a char as a single-character string. Other formats may
            // represent this differently.
            fn serialize_char(self, _v: char) -> Result<()> {
                self.serialize_u64(0)?;
                self.text_decoder()?;
                self.include_once(StdFuncs::CHAR | StdFuncs::SER, |this| {
                    this.output.push_str(include_str!("./js/ser_char.js"));
                    Ok(())
                })
            }

            // This only works for strings that don't require escape sequences but you
            // get the idea. For example it would emit invalid JSON if the input string
            // contains a '"' character.
            fn serialize_str(self, _v: &str) -> Result<()> {
                self.serialize_u64(0)?;
                self.text_decoder()?;
                self.include_once(StdFuncs::STRING | StdFuncs::SER, |this| {
                    this.output.push_str(include_str!("./js/ser_string.js"));
                    Ok(())
                })
            }

            // Serialize a byte array as an array of bytes. Could also use a base64
            // string here. Binary formats will typically represent byte arrays more
            // compactly.
            fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
                self.serialize_u64(0)?;
                self.include_once(StdFuncs::BYTES | StdFuncs::SER, |this| {
                    this.output.push_str(include_str!("./js/ser_bytes.js"));
                    Ok(())
                })
            }

            // An absent optional is represented as the JSON `null`.
            fn serialize_none(self) -> Result<()> {
                self.serialize_u32(0)?;
                self.include_once(StdFuncs::OPTION | StdFuncs::SER, |this| {
                    this.output.push_str(include_str!("./js/ser_option.js"));
                    Ok(())
                })
            }

            // A present optional is represented as just the contained value. Note that
            // this is a lossy representation. For example the values `Some(())` and
            // `None` both serialize as just `null`. Unfortunately this is typically
            // what people expect when working with JSON. Other formats are encouraged
            // to behave more intelligently if possible.
            fn serialize_some<T>(self, value: &T) -> Result<()>
            where
                T: ?Sized + Serialize,
            {
                self.serialize_none()
            }

            // In Serde, unit means an anonymous value containing no data. Map this to
            // JSON as `null`.
            fn serialize_unit(self) -> Result<()> {
                Ok(())
            }

            // Unit struct means a named value containing no data. Again, since there is
            // no data, map this to JSON as `null`. There is no need to serialize the
            // name in most formats.
            fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
                self.serialize_unit()
            }

            // When serializing a unit variant (or any other kind of variant), formats
            // can choose whether to keep track of it by index or by name. Binary
            // formats typically use the index of the variant and human-readable formats
            // typically use the name.
            fn serialize_unit_variant(
                self,
                _name: &'static str,
                _variant_index: u32,
                variant: &'static str,
            ) -> Result<()> {
                // self.serialize_str(variant)
                todo!()
            }

            // As is done here, serializers are encouraged to treat newtype structs as
            // insignificant wrappers around the data they contain.
            fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
            where
                T: ?Sized + Serialize,
            {
                value.serialize(self)
            }

            // Note that newtype variant (and all of the other variant serialization
            // methods) refer exclusively to the "externally tagged" enum
            // representation.
            //
            // Serialize this to JSON in externally tagged form as `{ NAME: VALUE }`.
            fn serialize_newtype_variant<T>(
                self,
                name: &'static str,
                _variant_index: u32,
                variant: &'static str,
                value: &T,
            ) -> Result<()>
            where
                T: ?Sized + Serialize,
            {
                self.output.write_fmt(format_args!("function serialize{name}(out, val) {{
                    throw new Error(\"unknown variant case\")
                }}"))?;

                variant.serialize(&mut *self)?;
                value.serialize(&mut *self)?;
                Ok(())
            }

            // Now we get to the serialization of compound types.
            //
            // The start of the sequence, each value, and the end are three separate
            // method calls. This one is responsible only for serializing the start,
            // which in JSON is `[`.
            //
            // The length of the sequence may or may not be known ahead of time. This
            // doesn't make a difference in JSON because the length is not represented
            // explicitly in the serialized form. Some serializers may only be able to
            // support sequences for which the length is known up front.
            fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
                self.serialize_u64(0)?;
                self.include_once(StdFuncs::LIST | StdFuncs::SER, |this| {
                    this.output.push_str(include_str!("./js/ser_list.js"));
                    Ok(())
                })?;
                Ok(self)
            }

            // Tuples look just like sequences in JSON. Some formats may be able to
            // represent tuples more efficiently by omitting the length, since tuple
            // means that the corresponding `Deserialize implementation will know the
            // length without needing to look at the serialized data.
            fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
                self.serialize_seq(Some(len))
            }

            // Tuple structs look just like sequences in JSON.
            fn serialize_tuple_struct(
                self,
                _name: &'static str,
                len: usize,
            ) -> Result<Self::SerializeTupleStruct> {
                self.serialize_seq(Some(len))
            }

            // Tuple variants are represented in JSON as `{ NAME: [DATA...] }`. Again
            // this method is only responsible for the externally tagged representation.
            fn serialize_tuple_variant(
                self,
                _name: &'static str,
                _variant_index: u32,
                variant: &'static str,
                _len: usize,
            ) -> Result<Self::SerializeTupleVariant> {
                variant.serialize(&mut *self)?;
                Ok(self)
            }

            // Maps are represented in JSON as `{ K: V, K: V, ... }`.
            fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
                Ok(self)
            }

            // Structs look just like maps in JSON. In particular, JSON requires that we
            // serialize the field names of the struct. Other formats may be able to
            // omit the field names when serializing structs because the corresponding
            // Deserialize implementation is required to know what the keys are without
            // looking at the serialized data.
            fn serialize_struct(
                self,
                _name: &'static str,
                len: usize,
            ) -> Result<Self::SerializeStruct> {
                self.serialize_map(Some(len))
            }

            // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }`.
            // This is the externally tagged representation.
            fn serialize_struct_variant(
                self,
                _name: &'static str,
                _variant_index: u32,
                variant: &'static str,
                _len: usize,
            ) -> Result<Self::SerializeStructVariant> {
                variant.serialize(&mut *self)?;
                Ok(self)
            }
        }

        // The following 7 impls deal with the serialization of compound types like
        // sequences and maps. Serialization of such types is begun by a Serializer
        // method and followed by zero or more calls to serialize individual elements of
        // the compound type and one call to end the compound type.
        //
        // This impl is SerializeSeq so these methods are called after `serialize_seq`
        // is called on the Serializer.
        impl<'a> ser::SerializeSeq for &'a mut Serializer {
            // Must match the `Ok` type of the serializer.
            type Ok = ();
            // Must match the `Error` type of the serializer.
            type Error = Error;

            // Serialize a single element of the sequence.
            fn serialize_element<T>(&mut self, value: &T) -> Result<()>
            where
                T: ?Sized + Serialize,
            {
                value.serialize(&mut **self)
            }

            // Close the sequence.
            fn end(self) -> Result<()> {
                Ok(())
            }
        }

        // Same thing but for tuples.
        impl<'a> ser::SerializeTuple for &'a mut Serializer {
            type Ok = ();
            type Error = Error;

            fn serialize_element<T>(&mut self, value: &T) -> Result<()>
            where
                T: ?Sized + Serialize,
            {
                value.serialize(&mut **self)
            }

            fn end(self) -> Result<()> {
                Ok(())
            }
        }

        // Same thing but for tuple structs.
        impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
            type Ok = ();
            type Error = Error;

            fn serialize_field<T>(&mut self, value: &T) -> Result<()>
            where
                T: ?Sized + Serialize,
            {
                value.serialize(&mut **self)
            }

            fn end(self) -> Result<()> {
                Ok(())
            }
        }

        // Tuple variants are a little different. Refer back to the
        // `serialize_tuple_variant` method above:
        //
        //    self.output += "{";
        //    variant.serialize(&mut *self)?;
        //    self.output += ":[";
        //
        // So the `end` method in this impl is responsible for closing both the `]` and
        // the `}`.
        impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
            type Ok = ();
            type Error = Error;

            fn serialize_field<T>(&mut self, value: &T) -> Result<()>
            where
                T: ?Sized + Serialize,
            {
                value.serialize(&mut **self)
            }

            fn end(self) -> Result<()> {
                Ok(())
            }
        }

        // Some `Serialize` types are not able to hold a key and value in memory at the
        // same time so `SerializeMap` implementations are required to support
        // `serialize_key` and `serialize_value` individually.
        //
        // There is a third optional method on the `SerializeMap` trait. The
        // `serialize_entry` method allows serializers to optimize for the case where
        // key and value are both available simultaneously. In JSON it doesn't make a
        // difference so the default behavior for `serialize_entry` is fine.
        impl<'a> ser::SerializeMap for &'a mut Serializer {
            type Ok = ();
            type Error = Error;

            // The Serde data model allows map keys to be any serializable type. JSON
            // only allows string keys so the implementation below will produce invalid
            // JSON if the key serializes as something other than a string.
            //
            // A real JSON serializer would need to validate that map keys are strings.
            // This can be done by using a different Serializer to serialize the key
            // (instead of `&mut **self`) and having that other serializer only
            // implement `serialize_str` and return an error on any other data type.
            fn serialize_key<T>(&mut self, key: &T) -> Result<()>
            where
                T: ?Sized + Serialize,
            {
                key.serialize(&mut **self)
            }

            // It doesn't make a difference whether the colon is printed at the end of
            // `serialize_key` or at the beginning of `serialize_value`. In this case
            // the code is a bit simpler having it here.
            fn serialize_value<T>(&mut self, value: &T) -> Result<()>
            where
                T: ?Sized + Serialize,
            {
                value.serialize(&mut **self)
            }

            fn end(self) -> Result<()> {
                Ok(())
            }
        }

        // Structs are like maps in which the keys are constrained to be compile-time
        // constant strings.
        impl<'a> ser::SerializeStruct for &'a mut Serializer {
            type Ok = ();
            type Error = Error;

            fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
            where
                T: ?Sized + Serialize,
            {
                key.serialize(&mut **self)?;
                value.serialize(&mut **self)
            }

            fn end(self) -> Result<()> {
                Ok(())
            }
        }

        // Similar to `SerializeTupleVariant`, here the `end` method is responsible for
        // closing both of the curly braces opened by `serialize_struct_variant`.
        impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
            type Ok = ();
            type Error = Error;

            fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
            where
                T: ?Sized + Serialize,
            {
                key.serialize(&mut **self)?;
                value.serialize(&mut **self)
            }

            fn end(self) -> Result<()> {
                Ok(())
            }
        }
    }
}
