const invoke = window.__TAURI_INVOKE__;
if (!window.__TAURI_BINDGEN_VERSION_CHECK__) {
  invoke("plugin|integers:279b557e344c2e05853f5c89d6d511dc").catch(() =>
    console.error(
      "The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`."
    )
  );
}

/**
 * @param {number} x
 */
export async function a1(x) {
  await invoke("plugin:integers|a1", { x: x });
}
/**
 * @param {number} x
 */
export async function a2(x) {
  await invoke("plugin:integers|a2", { x: x });
}
/**
 * @param {number} x
 */
export async function a3(x) {
  await invoke("plugin:integers|a3", { x: x });
}
/**
 * @param {number} x
 */
export async function a4(x) {
  await invoke("plugin:integers|a4", { x: x });
}
/**
 * @param {number} x
 */
export async function a5(x) {
  await invoke("plugin:integers|a5", { x: x });
}
/**
 * @param {number} x
 */
export async function a6(x) {
  await invoke("plugin:integers|a6", { x: x });
}
/**
 * @param {bigint} x
 */
export async function a7(x) {
  await invoke("plugin:integers|a7", { x: x });
}
/**
 * @param {bigint} x
 */
export async function a8(x) {
  await invoke("plugin:integers|a8", { x: x });
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
  const result = await invoke("plugin:integers|r1");
  return result;
}
/**
 * @returns {Promise<number>}
 */
export async function r2() {
  const result = await invoke("plugin:integers|r2");
  return result;
}
/**
 * @returns {Promise<number>}
 */
export async function r3() {
  const result = await invoke("plugin:integers|r3");
  return result;
}
/**
 * @returns {Promise<number>}
 */
export async function r4() {
  const result = await invoke("plugin:integers|r4");
  return result;
}
/**
 * @returns {Promise<number>}
 */
export async function r5() {
  const result = await invoke("plugin:integers|r5");
  return result;
}
/**
 * @returns {Promise<number>}
 */
export async function r6() {
  const result = await invoke("plugin:integers|r6");
  return result;
}
/**
 * @returns {Promise<bigint>}
 */
export async function r7() {
  const result = await invoke("plugin:integers|r7");
  return result;
}
/**
 * @returns {Promise<bigint>}
 */
export async function r8() {
  const result = await invoke("plugin:integers|r8");
  return result;
}
/**
 * @returns {Promise<[bigint, number]>}
 */
export async function pairRet() {
  const result = await invoke("plugin:integers|pair_ret");
  return result;
}
