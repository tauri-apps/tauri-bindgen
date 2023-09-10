use futures_util::stream::StreamExt;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;

tauri_bindgen_guest_rust::generate!({
    path: "greet.wit"
});

#[component]
pub fn App<G: Html>(cx: Scope) -> View<G> {
    let name = create_signal(cx, String::new());
    let greet_msg = create_signal(cx, String::new());

    spawn_local_scoped(cx, async move {
        log::debug!("starting stream");

        let mut stream = crate::greet2::greet::greet("world").await;

        while let Some(msg) = stream.next().await {
            log::debug!("got msg: {:?}", msg);
        }
    });

    let do_greet = move |_| {
        spawn_local_scoped(cx, async move {
            let new_msg = greet::greet(&name.get()).await;

            greet_msg.set(new_msg);
        })
    };

    view! { cx,
        main(class="container") {
            div(class="row") {
                input(id="greet-input",bind:value=name,placeholder="Enter a name...")
                button(type="button",on:click=do_greet) {
                    "Greet"
                }
            }
            p {
                b {
                    (greet_msg.get())
                }
            }
        }
    }
}
