const invoke = window.__TAURI_INVOKE__;
if (!window.__TAURI_BINDGEN_VERSION_CHECK__) {
  invoke("plugin|chars:678374cfb5cdb2b5ba845e4b559f402a").catch(() =>
    console.error(
      "The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`."
    )
  );
}

/**
 * A function that accepts a character
 * @param {string} x
 */
export async function takeChar(x) {
  await invoke("plugin:chars|take_char", { x: x });
}
/**
 * A function that returns a character
 * @returns {Promise<string>}
 */
export async function returnChar() {
  const result = await invoke("plugin:chars|return_char");
  return result;
}
