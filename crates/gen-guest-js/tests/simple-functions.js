const invoke = window.__TAURI_INVOKE__;
const idlHash = "ebb2d6f0441e00a0";
export async function f1() {
  await invoke("plugin:simple|f1", { idlHash });
}
/**
 * @param {number} a
 */
export async function f2(a) {
  await invoke("plugin:simple|f2", { idlHash, a: a });
}
/**
 * @param {number} a
 * @param {number} b
 */
export async function f3(a, b) {
  await invoke("plugin:simple|f3", { idlHash, a: a, b: b });
}
/**
 * @returns {Promise<number>}
 */
export async function f4() {
  const result = await invoke("plugin:simple|f4", { idlHash });
  return result;
}
/**
 * @returns {Promise<[number, number]>}
 */
export async function f5() {
  const result = await invoke("plugin:simple|f5", { idlHash });
  return result;
}
/**
 * @param {number} a
 * @param {number} b
 * @param {number} c
 * @returns {Promise<[number, number, number]>}
 */
export async function f6(a, b, c) {
  const result = await invoke("plugin:simple|f6", {
    idlHash,
    a: a,
    b: b,
    c: c,
  });
  return result;
}
