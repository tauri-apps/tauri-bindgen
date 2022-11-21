const { invoke } = window.__TAURI__.tauri;
export async function f1() {
  await invoke("plugin:imports|f1");
}
/**
 * @param {number} a
 */
export async function f2(a) {
  await invoke("plugin:imports|f2", { a: a });
}
/**
 * @param {number} a
 * @param {number} b
 */
export async function f3(a, b) {
  await invoke("plugin:imports|f3", { a: a, b: b });
}
/**
 * @returns {Promise<number>}
 */
export async function f4() {
  const result = await invoke("plugin:imports|f4");
  return result;
}
/**
 * @returns {Promise<[number, number]>}
 */
export async function f5() {
  const result = await invoke("plugin:imports|f5");
  return result;
}
/**
 * @param {number} a
 * @param {number} b
 * @param {number} c
 * @returns {Promise<[number, number, number]>}
 */
export async function f6(a, b, c) {
  const result = await invoke("plugin:imports|f6", { a: a, b: b, c: c });
  return result;
}
