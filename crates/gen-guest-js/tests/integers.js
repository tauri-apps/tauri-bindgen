const invoke = window.__TAURI_INVOKE__;
const idlHash = "279b557e344c2e05";
/**
 * @param {number} x
 */
export async function a1(x) {
  await invoke("plugin:integers|a1", { idlHash, x: x });
}
/**
 * @param {number} x
 */
export async function a2(x) {
  await invoke("plugin:integers|a2", { idlHash, x: x });
}
/**
 * @param {number} x
 */
export async function a3(x) {
  await invoke("plugin:integers|a3", { idlHash, x: x });
}
/**
 * @param {number} x
 */
export async function a4(x) {
  await invoke("plugin:integers|a4", { idlHash, x: x });
}
/**
 * @param {number} x
 */
export async function a5(x) {
  await invoke("plugin:integers|a5", { idlHash, x: x });
}
/**
 * @param {number} x
 */
export async function a6(x) {
  await invoke("plugin:integers|a6", { idlHash, x: x });
}
/**
 * @param {bigint} x
 */
export async function a7(x) {
  await invoke("plugin:integers|a7", { idlHash, x: x });
}
/**
 * @param {bigint} x
 */
export async function a8(x) {
  await invoke("plugin:integers|a8", { idlHash, x: x });
}
/**
 * @param {number} p1
 * @param {number} p2
 * @param {number} p3
 * @param {number} p4
 * @param {number} p5
 * @param {number} p6
 * @param {bigint} p7
 * @param {bigint} p8
 */
export async function a9(p1, p2, p3, p4, p5, p6, p7, p8) {
  await invoke("plugin:integers|a9", {
    idlHash,
    p1: p1,
    p2: p2,
    p3: p3,
    p4: p4,
    p5: p5,
    p6: p6,
    p7: p7,
    p8: p8,
  });
}
/**
 * @returns {Promise<number>}
 */
export async function r1() {
  const result = await invoke("plugin:integers|r1", { idlHash });
  return result;
}
/**
 * @returns {Promise<number>}
 */
export async function r2() {
  const result = await invoke("plugin:integers|r2", { idlHash });
  return result;
}
/**
 * @returns {Promise<number>}
 */
export async function r3() {
  const result = await invoke("plugin:integers|r3", { idlHash });
  return result;
}
/**
 * @returns {Promise<number>}
 */
export async function r4() {
  const result = await invoke("plugin:integers|r4", { idlHash });
  return result;
}
/**
 * @returns {Promise<number>}
 */
export async function r5() {
  const result = await invoke("plugin:integers|r5", { idlHash });
  return result;
}
/**
 * @returns {Promise<number>}
 */
export async function r6() {
  const result = await invoke("plugin:integers|r6", { idlHash });
  return result;
}
/**
 * @returns {Promise<bigint>}
 */
export async function r7() {
  const result = await invoke("plugin:integers|r7", { idlHash });
  return result;
}
/**
 * @returns {Promise<bigint>}
 */
export async function r8() {
  const result = await invoke("plugin:integers|r8", { idlHash });
  return result;
}
/**
 * @returns {Promise<[bigint, number]>}
 */
export async function pairRet() {
  const result = await invoke("plugin:integers|pair_ret", { idlHash });
  return result;
}
