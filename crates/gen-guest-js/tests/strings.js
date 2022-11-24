const invoke = window.__TAURI_INVOKE__;
const idlHash = "16c3ebd2deefea81";
/**
 * @param {string} x
 */
export async function a(x) {
  await invoke("plugin:strings|a", { idlHash, x: x });
}
/**
 * @returns {Promise<string>}
 */
export async function b() {
  const result = await invoke("plugin:strings|b", { idlHash });
  return result;
}
/**
 * @param {string} a
 * @param {string} b
 * @returns {Promise<string>}
 */
export async function c(a, b) {
  const result = await invoke("plugin:strings|c", { idlHash, a: a, b: b });
  return result;
}
