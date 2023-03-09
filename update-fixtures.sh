cargo build --features unstable
for i in wit/*.wit; do target/debug/tauri-bindgen host --tracing --out-dir crates/gen-host/tests --fmt $i; done

for i in wit/*.wit; do target/debug/tauri-bindgen markdown --out-dir crates/gen-markdown/tests $i; done

for i in wit/*.wit; do target/debug/tauri-bindgen guest rust --out-dir crates/gen-guest-rust/tests --fmt $i; done
for i in wit/*.wit; do target/debug/tauri-bindgen guest javascript --out-dir crates/gen-guest-js/tests $i; done
for i in wit/*.wit; do target/debug/tauri-bindgen guest typescript --out-dir crates/gen-guest-ts/tests $i; done
# for i in wit/*.wit; do target/debug/tauri-bindgen guest rescript --fmt --out-dir crates/gen-guest-rescript/tests $i; done

