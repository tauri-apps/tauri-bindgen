const { invoke } = window.__TAURI__.tauri;
/**
 * @param {string} x
 */
export async function a(x) {
  await invoke("plugin:imports|a", { x: x });
}
/**
 * @returns {Promise<string>}
 */
export async function b() {
  const result = await invoke("plugin:imports|b");
  return result;
}
/**
 * @param {string} a
 * @param {string} b
 * @returns {Promise<string>}
 */
export async function c(a, b) {
  const result = await invoke("plugin:imports|c", { a: a, b: b });
  return result;
}
