mod roundtrip;

extern crate console_error_panic_hook;
use std::future::Future;
use std::panic;
use sycamore::prelude::*;
use sycamore::suspense::Suspense;

#[cfg(feature = "ci")]
async fn exit_with_error(e: String) {
    use serde::Serialize;

    #[derive(Serialize)]
    struct Args {
        e: String,
    }

    tauri_sys::tauri::invoke::<_, ()>("exit_with_error", &Args { e })
        .await
        .unwrap();
}

#[derive(Props)]
pub struct TestProps<'a, F>
where
    F: Future<Output = anyhow::Result<()>> + 'a,
{
    name: &'a str,
    test: F,
}

#[component]
pub async fn TestInner<'a, G: Html, F>(cx: Scope<'a>, props: TestProps<'a, F>) -> View<G>
where
    F: Future<Output = anyhow::Result<()>> + 'a,
{
    let res = props.test.await;

    view! { cx,
        tr {
            td { code { (props.name.to_string()) } }
            td { (if let Err(e) = &res {
                    #[cfg(feature = "ci")]
                    {
                        wasm_bindgen_futures::spawn_local(exit_with_error(e.to_string()));
                        unreachable!()
                    }
                    #[cfg(not(feature = "ci"))]
                    format!("❌ {:?}", e)
                } else {
                    format!("✅")
                })
            }
        }
    }
}

#[component]
pub fn Test<'a, G: Html, F>(cx: Scope<'a>, props: TestProps<'a, F>) -> View<G>
where
    F: Future<Output = anyhow::Result<()>> + 'a,
{
    let fallback = view! { cx,
        tr {
            td { code { (props.name.to_string()) } }
            td {
                span(class="loader") { "⏳" }
            }
        }
    };

    view! { cx,
        Suspense(fallback=fallback) {
            TestInner(name=props.name, test=props.test)
        }
    }
}

#[component]
pub async fn Terminate<'a, G: Html>(cx: Scope<'a>) -> View<G> {
    #[cfg(feature = "ci")]
    sycamore::suspense::await_suspense(cx, async {
        tauri_sys::process::exit(0).await;
    })
    .await;

    view! {
        cx,
    }
}

// static LOGGER: TauriLogger = TauriLogger;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    // log::set_logger(&LOGGER)
    //     .map(|()| log::set_max_level(LevelFilter::Trace))
    //     .unwrap();

    panic::set_hook(Box::new(|info| {
        console_error_panic_hook::hook(info);

        #[cfg(feature = "ci")]
        wasm_bindgen_futures::spawn_local(exit_with_error(format!("{}", info)));
    }));

    sycamore::render(|cx| {
        view! { cx,
            table {
                tbody {
                    Test(name="empty",test=roundtrip::empty())
                    Test(name="record_scalars",test=roundtrip::record_scalars())
                    Test(name="record_really_flags",test=roundtrip::record_really_flags())
                    Test(name="record_aggregates",test=roundtrip::record_aggregates())
                    Test(name="flag1",test=roundtrip::flag1())
                    Test(name="float32",test=roundtrip::float32())
                    Test(name="float64",test=roundtrip::float64())
                    Test(name="u8",test=roundtrip::u8())
                    Test(name="s8",test=roundtrip::s8())
                    Test(name="u16",test=roundtrip::u16())
                    Test(name="s16",test=roundtrip::s16())
                    Test(name="u32",test=roundtrip::u32())
                    Test(name="s32",test=roundtrip::s32())
                    Test(name="u64",test=roundtrip::u64())
                    Test(name="s64",test=roundtrip::s64())

                    Test(name="list_u8",test=roundtrip::list_u8())
                    Test(name="list_u16",test=roundtrip::list_u16())
                    Test(name="list_u32",test=roundtrip::list_u32())
                    Test(name="list_u64",test=roundtrip::list_u64())

                    Test(name="list_s8",test=roundtrip::list_s8())
                    Test(name="list_s16",test=roundtrip::list_s16())
                    Test(name="list_s32",test=roundtrip::list_s32())
                    Test(name="list_s64",test=roundtrip::list_s64())

                    Test(name="list_float32",test=roundtrip::list_float32())
                    Test(name="list_float64",test=roundtrip::list_float64())
                    Test(name="tuple_list",test=roundtrip::tuple_list())
                    Test(name="string_list",test=roundtrip::string_list())
                    Test(name="tuple_string_list",test=roundtrip::tuple_string_list())

                    Test(name="all_integers",test=roundtrip::all_integers())
                    Test(name="all_floats",test=roundtrip::all_floats())
                    Test(name="all_text",test=roundtrip::all_text())

                    Test(name="options",test=roundtrip::options())
                    Test(name="results",test=roundtrip::results())

                    Terminate
                }
            }
        }
    });
}
