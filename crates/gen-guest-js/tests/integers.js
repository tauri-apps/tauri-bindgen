const { invoke } = window.__TAURI__.tauri;
/**
 * @param {number} x
 */
export async function a1(x) {
  await invoke("plugin:imports|a1", { x: x });
}
/**
 * @param {number} x
 */
export async function a2(x) {
  await invoke("plugin:imports|a2", { x: x });
}
/**
 * @param {number} x
 */
export async function a3(x) {
  await invoke("plugin:imports|a3", { x: x });
}
/**
 * @param {number} x
 */
export async function a4(x) {
  await invoke("plugin:imports|a4", { x: x });
}
/**
 * @param {number} x
 */
export async function a5(x) {
  await invoke("plugin:imports|a5", { x: x });
}
/**
 * @param {number} x
 */
export async function a6(x) {
  await invoke("plugin:imports|a6", { x: x });
}
/**
 * @param {bigint} x
 */
export async function a7(x) {
  await invoke("plugin:imports|a7", { x: x });
}
/**
 * @param {bigint} x
 */
export async function a8(x) {
  await invoke("plugin:imports|a8", { x: x });
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
  await invoke("plugin:imports|a9", {
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
  const result = await invoke("plugin:imports|r1");
  return result;
}
/**
 * @returns {Promise<number>}
 */
export async function r2() {
  const result = await invoke("plugin:imports|r2");
  return result;
}
/**
 * @returns {Promise<number>}
 */
export async function r3() {
  const result = await invoke("plugin:imports|r3");
  return result;
}
/**
 * @returns {Promise<number>}
 */
export async function r4() {
  const result = await invoke("plugin:imports|r4");
  return result;
}
/**
 * @returns {Promise<number>}
 */
export async function r5() {
  const result = await invoke("plugin:imports|r5");
  return result;
}
/**
 * @returns {Promise<number>}
 */
export async function r6() {
  const result = await invoke("plugin:imports|r6");
  return result;
}
/**
 * @returns {Promise<bigint>}
 */
export async function r7() {
  const result = await invoke("plugin:imports|r7");
  return result;
}
/**
 * @returns {Promise<bigint>}
 */
export async function r8() {
  const result = await invoke("plugin:imports|r8");
  return result;
}
/**
 * @returns {Promise<[bigint, number]>}
 */
export async function pairRet() {
  const result = await invoke("plugin:imports|pair_ret");
  return result;
}
