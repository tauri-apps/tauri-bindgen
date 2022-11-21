const { invoke } = window.__TAURI__.tauri;
export async function kebabCase() {
  await invoke("plugin:imports|kebab_case");
}
/**
 * @param {LudicrousSpeed} x
 */
export async function foo(x) {
  await invoke("plugin:imports|foo", { x: x });
}
export async function functionWithDashes() {
  await invoke("plugin:imports|function_with_dashes");
}
export async function functionWithNoWeirdCharacters() {
  await invoke("plugin:imports|function_with_no_weird_characters");
}
export async function apple() {
  await invoke("plugin:imports|apple");
}
export async function applePear() {
  await invoke("plugin:imports|apple_pear");
}
export async function applePearGrape() {
  await invoke("plugin:imports|apple_pear_grape");
}
export async function a0() {
  await invoke("plugin:imports|a0");
}
export async function isXml() {
  await invoke("plugin:imports|is_xml");
}
export async function explicit() {
  await invoke("plugin:imports|explicit");
}
export async function explicitKebab() {
  await invoke("plugin:imports|explicit_kebab");
}
export async function bool() {
  await invoke("plugin:imports|bool");
}
