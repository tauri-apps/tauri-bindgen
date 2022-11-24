const invoke = window.__TAURI_INVOKE__;
const idlHash = "d40a3203ef48115d";
/**
 * @param {Uint32Array} l
 */
export async function simpleList1(l) {
  await invoke("plugin:simple_lists|simple_list1", { idlHash, l: l });
}
/**
 * @returns {Promise<Uint32Array>}
 */
export async function simpleList2() {
  const result = await invoke("plugin:simple_lists|simple_list2", { idlHash });
  return result;
}
/**
 * @param {Uint32Array[]} l
 * @returns {Promise<Uint32Array[]>}
 */
export async function simpleList4(l) {
  const result = await invoke("plugin:simple_lists|simple_list4", {
    idlHash,
    l: l,
  });
  return result;
}
