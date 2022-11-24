const invoke = window.__TAURI_INVOKE__;
if (!window.__TAURI_BINDGEN_VERSION_CHECK__) {
  invoke("plugin|floats:b2ded0ef970e65969239249842d626ce").catch(() =>
    console.error(
      "The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`."
    )
  );
}

/**
 * @param {number} x
 */
export async function float32Param(x) {
  await invoke("plugin:floats|float32_param", { x: x });
}
/**
 * @param {number} x
 */
export async function float64Param(x) {
  await invoke("plugin:floats|float64_param", { x: x });
}
/**
 * @returns {Promise<number>}
 */
export async function float32Result() {
  const result = await invoke("plugin:floats|float32_result");
  return result;
}
/**
 * @returns {Promise<number>}
 */
export async function float64Result() {
  const result = await invoke("plugin:floats|float64_result");
  return result;
}
