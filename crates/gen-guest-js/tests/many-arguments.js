const invoke = window.__TAURI_INVOKE__;
if (!window.__TAURI_BINDGEN_VERSION_CHECK__) {
  invoke("plugin|manyarg:92d5120c899c41cc0c9bb8a02b370a08").catch(() =>
    console.error(
      "The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`."
    )
  );
}

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
  await invoke("plugin:manyarg|big_argument", { x: x });
}
