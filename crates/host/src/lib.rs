use std::{collections::HashMap, marker::PhantomData};

use serde::{de::DeserializeOwned, Serialize};
use tauri::{AppHandle, Runtime};
pub use tauri_bindgen_host_macro::*;

#[doc(hidden)]
pub use {anyhow, async_trait::async_trait, bitflags, serde, tauri, tracing};

pub struct Context<'a, T, Rt: Runtime> {
    pub(crate) state: &'a mut T,
    app_handle: AppHandle<Rt>,
}

type Function<T, Rt> = dyn Fn(Context<'_, T, Rt>, serde_json::Value);

// trait IntoFunc<T, Params, Results, Rt: Runtime>: Send + Sync {
//     #[doc(hidden)]
//     fn into_func(self) -> Box<Function<T, Rt>>;
// }

pub trait IntoFunc<T, Params, Results, Rt: Runtime>: Send + Sync + 'static {
    #[doc(hidden)]
    fn into_func(self) -> Box<Function<T, Rt>>;
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

macro_rules! impl_into_func {
    ($num:tt $($args:ident)*) => {
        // Implement for functions without a leading `&Caller` parameter,
        // delegating to the implementation below which does have the leading
        // `Caller` parameter.
        #[allow(non_snake_case)]
        impl<T, F, $($args,)* R, Rt> IntoFunc<T, ($($args,)*), R, Rt> for F
        where
            F: Fn($($args),*) -> R + Send + Sync + 'static,
            $($args: DeserializeOwned,)*
            R: Serialize,
            Rt: Runtime
        {
            fn into_func(self) -> Box<Function<T, Rt>> {
                let f = move |_: Context<'_, T, Rt>, $($args:$args),*| {
                    self($($args),*)
                };

                f.into_func()
            }
        }

        #[allow(non_snake_case)]
        impl<T, F, $($args,)* R, Rt> IntoFunc<T, (Context<'_, T, Rt>, $($args,)*), R, Rt> for F
        where
            F: Fn(Context<'_, T, Rt>, $($args),*) -> R + Send + Sync + 'static,
            $($args: DeserializeOwned,)*
            R: Serialize,
            Rt: Runtime
        {
            fn into_func(self) -> Box<Function<T, Rt>> {
                Box::new(move |ctx, args| {
                    let ($($args,)*) = serde_json::from_value(args).unwrap();

                    let ret = self(ctx, $($args),*);

                    todo!();
                })
            }
        }
    }
}

for_each_function_signature!(impl_into_func);

struct Router<'a, T, Rt: Runtime> {
    _m: PhantomData<(T, Rt)>,
    state: &'a mut T,
    app_handle: AppHandle<Rt>,
    map: HashMap<String, Box<Function<T, Rt>>>,
}

impl<'a, T, Rt: Runtime> Router<'a, T, Rt> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn func_sync<Params, Results>(
        &mut self,
        module: &str,
        name: &str,
        handler: impl IntoFunc<T, Params, Results, Rt>,
    ) {
        self.map
            .insert(format!("{}.{}", module, name), handler.into_func());
    }

    pub fn build(self) -> impl Fn(tauri::Invoke<Rt>) + 'a {
        move |invoke| {
            let handler = self.map.get(invoke.message.command()).unwrap();

            let ctx = Context {
                state: self.state,
                app_handle: self.app_handle.clone(),
            };

            todo!()
        }
    }
}
