const { invoke } = window.__TAURI__.tauri;
/**
 * A function that accepts a character
 * @param {string} x
 */
export async function takeChar(x) {
  await invoke("plugin:imports|take_char", { x: x });
}
/**
 * A function that returns a character
 * @returns {Promise<string>}
 */
export async function returnChar() {
  const result = await invoke("plugin:imports|return_char");
  return result;
}
