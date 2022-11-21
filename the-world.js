/**
 * A function that accepts a character
 * @param {string} x
 */
export async function takeChar(x) {
  await window.__TAURI__.tauri.invoke("plugin:imports|take_char", { x: x });
}
/**
 * A function that returns a character
 * @returns {Promise<string>}
 */
export async function returnChar() {
  const result = await window.__TAURI__.tauri.invoke(
    "plugin:imports|return_char"
  );
  return result;
}
