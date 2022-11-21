const { invoke } = window.__TAURI__.tauri;
/**
 * @param {number} x
 */
export async function float32Param(x) {
  await invoke("plugin:imports|float32_param", { x: x });
}
/**
 * @param {number} x
 */
export async function float64Param(x) {
  await invoke("plugin:imports|float64_param", { x: x });
}
/**
 * @returns {Promise<number>}
 */
export async function float32Result() {
  const result = await invoke("plugin:imports|float32_result");
  return result;
}
/**
 * @returns {Promise<number>}
 */
export async function float64Result() {
  const result = await invoke("plugin:imports|float64_result");
  return result;
}
