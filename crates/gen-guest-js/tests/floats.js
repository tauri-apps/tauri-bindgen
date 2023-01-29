const invoke = window.__TAURI_INVOKE__;
/**
* @param {number} x
* @returns {Promise<[]>}
*/
export async function float32Param(x) {
  const result = await invoke("plugin:979575fda4ffb8b9|float32_param",{x: x});
  return result
}
/**
* @param {number} x
* @returns {Promise<[]>}
*/
export async function float64Param(x) {
  const result = await invoke("plugin:979575fda4ffb8b9|float64_param",{x: x});
  return result
}
/**
* @returns {Promise<number>}
*/
export async function float32Result() {
  const result = await invoke("plugin:979575fda4ffb8b9|float32_result",);
  return result
}
/**
* @returns {Promise<number>}
*/
export async function float64Result() {
  const result = await invoke("plugin:979575fda4ffb8b9|float64_result",);
  return result
}

