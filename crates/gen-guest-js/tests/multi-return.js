const invoke = window.__TAURI_INVOKE__;
const idlHash = "d238f57052cdcb90";
export async function mra() {
  await invoke("plugin:multi_return|mra", { idlHash });
}
export async function mrb() {
  await invoke("plugin:multi_return|mrb", { idlHash });
}
/**
 * @returns {Promise<number>}
 */
export async function mrc() {
  const result = await invoke("plugin:multi_return|mrc", { idlHash });
  return result;
}
/**
 * @returns {Promise<number>}
 */
export async function mrd() {
  const result = await invoke("plugin:multi_return|mrd", { idlHash });
  return result;
}
/**
 * @returns {Promise<[number, number]>}
 */
export async function mre() {
  const result = await invoke("plugin:multi_return|mre", { idlHash });
  return result;
}
