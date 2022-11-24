const invoke = window.__TAURI_INVOKE__;
const idlHash = "92d5120c899c41cc";
/**
 * @param {bigint} a1
 * @param {bigint} a2
 * @param {bigint} a3
 * @param {bigint} a4
 * @param {bigint} a5
 * @param {bigint} a6
 * @param {bigint} a7
 * @param {bigint} a8
 * @param {bigint} a9
 * @param {bigint} a10
 * @param {bigint} a11
 * @param {bigint} a12
 * @param {bigint} a13
 * @param {bigint} a14
 * @param {bigint} a15
 * @param {bigint} a16
 */
export async function manyArgs(
  a1,
  a2,
  a3,
  a4,
  a5,
  a6,
  a7,
  a8,
  a9,
  a10,
  a11,
  a12,
  a13,
  a14,
  a15,
  a16
) {
  await invoke("plugin:manyarg|many_args", {
    idlHash,
    a1: a1,
    a2: a2,
    a3: a3,
    a4: a4,
    a5: a5,
    a6: a6,
    a7: a7,
    a8: a8,
    a9: a9,
    a10: a10,
    a11: a11,
    a12: a12,
    a13: a13,
    a14: a14,
    a15: a15,
    a16: a16,
  });
}
/**
 * @param {BigStruct} x
 */
export async function bigArgument(x) {
  await invoke("plugin:manyarg|big_argument", { idlHash, x: x });
}
