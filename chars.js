const invoke = window.__TAURI_INVOKE__;
/**
 * A function that accepts a character
 * @param {string} x
 */
export async function takeChar(x) {
  await invoke("plugin:chars|0", { 0: x });
}
/**
 * A function that returns a character
 * @returns {Promise<string>}
 */
export async function returnChar() {
  const result = await invoke("plugin:chars|1");
  return result;
}
