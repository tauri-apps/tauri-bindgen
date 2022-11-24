for i in tests/codegen/*.wit; do target/debug/tauri-bindgen-cli host --rustfmt --tracing --async --out-dir crates/gen-host/tests $i; done

for i in tests/codegen/*.wit; do target/debug/tauri-bindgen-cli guest rust --rustfmt --out-dir crates/gen-guest-rust/tests $i; done
for i in tests/codegen/*.wit; do target/debug/tauri-bindgen-cli guest javascript --prettier --out-dir crates/gen-guest-js/tests $i; done
for i in tests/codegen/*.wit; do target/debug/tauri-bindgen-cli guest typescript --prettier --out-dir crates/gen-guest-ts/tests $i; done
for i in tests/codegen/*.wit; do target/debug/tauri-bindgen-cli guest rescript --fmt --out-dir crates/gen-guest-rescript/tests $i; done

