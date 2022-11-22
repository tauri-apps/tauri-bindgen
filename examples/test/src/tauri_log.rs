use ::log::{Metadata, Record};

tauri_bindgen_guest_rust::generate!({
    path: "tauri-log.wit"
});

pub struct TauriLogger;
impl log::Log for TauriLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= ::log::Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let lvl = match record.level() {
                ::log::Level::Trace => 1,
                ::log::Level::Debug => 2,
                ::log::Level::Info => 3,
                ::log::Level::Warn => 4,
                ::log::Level::Error => 5,
            };
            let message = record.target().to_string();
            let location = format!("{}", record.clone().args());
            // let file = record.file().map(ToString::to_string);
            let line = record.line();

            wasm_bindgen_futures::spawn_local(async move {
                tauri_log::log(lvl, &message, &location, None, line).await;
            });
        }
    }

    fn flush(&self) {}
}
