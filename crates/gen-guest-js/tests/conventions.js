const invoke = window.__TAURI_INVOKE__;
const idlHash = "48646a1b1c089063";
export async function kebabCase() {
  await invoke("plugin:conventions|kebab_case", { idlHash });
}
/**
 * @param {LudicrousSpeed} x
 */
export async function foo(x) {
  await invoke("plugin:conventions|foo", { idlHash, x: x });
}
export async function functionWithDashes() {
  await invoke("plugin:conventions|function_with_dashes", { idlHash });
}
export async function functionWithNoWeirdCharacters() {
  await invoke("plugin:conventions|function_with_no_weird_characters", {
    idlHash,
  });
}
export async function apple() {
  await invoke("plugin:conventions|apple", { idlHash });
}
export async function applePear() {
  await invoke("plugin:conventions|apple_pear", { idlHash });
}
export async function applePearGrape() {
  await invoke("plugin:conventions|apple_pear_grape", { idlHash });
}
export async function a0() {
  await invoke("plugin:conventions|a0", { idlHash });
}
export async function isXml() {
  await invoke("plugin:conventions|is_xml", { idlHash });
}
export async function explicit() {
  await invoke("plugin:conventions|explicit", { idlHash });
}
export async function explicitKebab() {
  await invoke("plugin:conventions|explicit_kebab", { idlHash });
}
export async function bool() {
  await invoke("plugin:conventions|bool", { idlHash });
}
