const invoke = window.__TAURI_INVOKE__;
if (!window.__TAURI_BINDGEN_VERSION_CHECK__) {
  invoke("plugin|flegs:8ecd22d5a53ba1eb34b6d188f5479d66").catch(() =>
    console.error(
      "The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`."
    )
  );
}

/**
 * @param {Flag1} x
 * @returns {Promise<Flag1>}
 */
export async function roundtripFlag1(x) {
  const result = await invoke("plugin:flegs|roundtrip_flag1", { x: x });
  return result;
}
/**
 * @param {Flag2} x
 * @returns {Promise<Flag2>}
 */
export async function roundtripFlag2(x) {
  const result = await invoke("plugin:flegs|roundtrip_flag2", { x: x });
  return result;
}
/**
 * @param {Flag4} x
 * @returns {Promise<Flag4>}
 */
export async function roundtripFlag4(x) {
  const result = await invoke("plugin:flegs|roundtrip_flag4", { x: x });
  return result;
}
/**
 * @param {Flag8} x
 * @returns {Promise<Flag8>}
 */
export async function roundtripFlag8(x) {
  const result = await invoke("plugin:flegs|roundtrip_flag8", { x: x });
  return result;
}
/**
 * @param {Flag16} x
 * @returns {Promise<Flag16>}
 */
export async function roundtripFlag16(x) {
  const result = await invoke("plugin:flegs|roundtrip_flag16", { x: x });
  return result;
}
/**
 * @param {Flag32} x
 * @returns {Promise<Flag32>}
 */
export async function roundtripFlag32(x) {
  const result = await invoke("plugin:flegs|roundtrip_flag32", { x: x });
  return result;
}
/**
 * @param {Flag64} x
 * @returns {Promise<Flag64>}
 */
export async function roundtripFlag64(x) {
  const result = await invoke("plugin:flegs|roundtrip_flag64", { x: x });
  return result;
}
