const { invoke } = window.__TAURI__.tauri;
export async function mra() {
  await invoke("plugin:imports|mra");
}
export async function mrb() {
  await invoke("plugin:imports|mrb");
}
/**
 * @returns {Promise<number>}
 */
export async function mrc() {
  const result = await invoke("plugin:imports|mrc");
  return result;
}
/**
 * @returns {Promise<number>}
 */
export async function mrd() {
  const result = await invoke("plugin:imports|mrd");
  return result;
}
/**
 * @returns {Promise<[number, number]>}
 */
export async function mre() {
  const result = await invoke("plugin:imports|mre");
  return result;
}
