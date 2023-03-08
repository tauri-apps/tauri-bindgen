cargo build --features unstable
for i in tests/codegen/*.wit; do target/debug/tauri-bindgen-cli host --tracing --out-dir crates/gen-host/tests --fmt $i; done

for i in tests/codegen/*.wit; do target/debug/tauri-bindgen-cli markdown --out-dir crates/gen-markdown/tests $i; done

for i in tests/codegen/*.wit; do target/debug/tauri-bindgen-cli guest rust --out-dir crates/gen-guest-rust/tests --fmt $i; done
for i in tests/codegen/*.wit; do target/debug/tauri-bindgen-cli guest javascript --out-dir crates/gen-guest-js/tests $i; done
# for i in tests/codegen/*.wit; do target/debug/tauri-bindgen-cli guest typescript --out-dir crates/gen-guest-ts/tests $i; done
# for i in tests/codegen/*.wit; do target/debug/tauri-bindgen-cli guest rescript --fmt --out-dir crates/gen-guest-rescript/tests $i; done

