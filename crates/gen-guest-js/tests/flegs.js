const invoke = window.__TAURI_INVOKE__;
const idlHash = "8ecd22d5a53ba1eb";
/**
 * @param {Flag1} x
 * @returns {Promise<Flag1>}
 */
export async function roundtripFlag1(x) {
  const result = await invoke("plugin:flegs|roundtrip_flag1", {
    idlHash,
    x: x,
  });
  return result;
}
/**
 * @param {Flag2} x
 * @returns {Promise<Flag2>}
 */
export async function roundtripFlag2(x) {
  const result = await invoke("plugin:flegs|roundtrip_flag2", {
    idlHash,
    x: x,
  });
  return result;
}
/**
 * @param {Flag4} x
 * @returns {Promise<Flag4>}
 */
export async function roundtripFlag4(x) {
  const result = await invoke("plugin:flegs|roundtrip_flag4", {
    idlHash,
    x: x,
  });
  return result;
}
/**
 * @param {Flag8} x
 * @returns {Promise<Flag8>}
 */
export async function roundtripFlag8(x) {
  const result = await invoke("plugin:flegs|roundtrip_flag8", {
    idlHash,
    x: x,
  });
  return result;
}
/**
 * @param {Flag16} x
 * @returns {Promise<Flag16>}
 */
export async function roundtripFlag16(x) {
  const result = await invoke("plugin:flegs|roundtrip_flag16", {
    idlHash,
    x: x,
  });
  return result;
}
/**
 * @param {Flag32} x
 * @returns {Promise<Flag32>}
 */
export async function roundtripFlag32(x) {
  const result = await invoke("plugin:flegs|roundtrip_flag32", {
    idlHash,
    x: x,
  });
  return result;
}
/**
 * @param {Flag64} x
 * @returns {Promise<Flag64>}
 */
export async function roundtripFlag64(x) {
  const result = await invoke("plugin:flegs|roundtrip_flag64", {
    idlHash,
    x: x,
  });
  return result;
}
