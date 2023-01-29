mod app;

use app::App;

#[cfg(all(not(debug_assertions), not(feature = "ssg")))]
fn main() {
    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();

    sycamore::hydrate(App);
}

#[cfg(all(debug_assertions, not(feature = "ssg")))]
fn main() {
    console_error_panic_hook::set_once();

    tracing_wasm::set_as_global_default();

    sycamore::render(App);
}

#[cfg(feature = "ssg")]
fn main() {
    let out_dir = std::env::args().nth(1).unwrap();

    println!("out_dir {}", out_dir);

    let template = std::fs::read_to_string(format!("{}/index.html", out_dir)).unwrap();

    let html = sycamore::render_to_string(App);

    let html = template.replace("<!--app-html-->\n", &html);

    let path = format!("{}/index.html", out_dir);

    println!("Writing html to file \"{}\"", path);
    std::fs::write(path, html).unwrap();
}
