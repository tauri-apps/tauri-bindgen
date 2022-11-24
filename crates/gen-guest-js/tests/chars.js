const invoke = window.__TAURI_INVOKE__;
const idlHash = "678374cfb5cdb2b5";
/**
 * A function that accepts a character
 * @param {string} x
 */
export async function takeChar(x) {
  await invoke("plugin:chars|take_char", { idlHash, x: x });
}
/**
 * A function that returns a character
 * @returns {Promise<string>}
 */
export async function returnChar() {
  const result = await invoke("plugin:chars|return_char", { idlHash });
  return result;
}
