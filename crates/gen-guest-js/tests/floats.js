const invoke = window.__TAURI_INVOKE__;
const idlHash = "b2ded0ef970e6596";
/**
 * @param {number} x
 */
export async function float32Param(x) {
  await invoke("plugin:floats|float32_param", { idlHash, x: x });
}
/**
 * @param {number} x
 */
export async function float64Param(x) {
  await invoke("plugin:floats|float64_param", { idlHash, x: x });
}
/**
 * @returns {Promise<number>}
 */
export async function float32Result() {
  const result = await invoke("plugin:floats|float32_result", { idlHash });
  return result;
}
/**
 * @returns {Promise<number>}
 */
export async function float64Result() {
  const result = await invoke("plugin:floats|float64_result", { idlHash });
  return result;
}
