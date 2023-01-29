const invoke = window.__TAURI_INVOKE__;
/**
* @param {Uint32Array} l
* @returns {Promise<[]>}
*/
export async function simpleList1(l) {
  const result = await invoke("plugin:e8600e8d0423cbdb|simple_list1",{l: l});
  return result
}
/**
* @returns {Promise<Uint32Array>}
*/
export async function simpleList2() {
  const result = await invoke("plugin:e8600e8d0423cbdb|simple_list2",);
  return result
}
/**
* @param {Uint32Array[]} l
* @returns {Promise<Uint32Array[]>}
*/
export async function simpleList4(l) {
  const result = await invoke("plugin:e8600e8d0423cbdb|simple_list4",{l: l});
  return result
}

