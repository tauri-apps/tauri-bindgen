const invoke = window.__TAURI_INVOKE__;
export async function kebabCase() {
	await invoke("plugin:48646a1b1c089063|kebab_case");
}
/**
 * @param {LudicrousSpeed} x
 */
export async function foo(x) {
	await invoke("plugin:48646a1b1c089063|foo", { x: x });
}
export async function functionWithDashes() {
	await invoke("plugin:48646a1b1c089063|function_with_dashes");
}
export async function functionWithNoWeirdCharacters() {
	await invoke("plugin:48646a1b1c089063|function_with_no_weird_characters");
}
export async function apple() {
	await invoke("plugin:48646a1b1c089063|apple");
}
export async function applePear() {
	await invoke("plugin:48646a1b1c089063|apple_pear");
}
export async function applePearGrape() {
	await invoke("plugin:48646a1b1c089063|apple_pear_grape");
}
export async function a0() {
	await invoke("plugin:48646a1b1c089063|a0");
}
export async function isXml() {
	await invoke("plugin:48646a1b1c089063|is_xml");
}
export async function explicit() {
	await invoke("plugin:48646a1b1c089063|explicit");
}
export async function explicitKebab() {
	await invoke("plugin:48646a1b1c089063|explicit_kebab");
}
export async function bool() {
	await invoke("plugin:48646a1b1c089063|bool");
}

