const invoke = window.__TAURI_INVOKE__;
if (!window.__TAURI_BINDGEN_VERSION_CHECK__) {
	invoke("plugin|conventions:48646a1b1c089063e7b03a4c1dd9f5ad").catch(() =>
		console.error(
			"The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`.",
		),
	);
}

export async function kebabCase() {
	await invoke("plugin:conventions|kebab_case");
}
/**
 * @param {LudicrousSpeed} x
 */
export async function foo(x) {
	await invoke("plugin:conventions|foo", { x: x });
}
export async function functionWithDashes() {
	await invoke("plugin:conventions|function_with_dashes");
}
export async function functionWithNoWeirdCharacters() {
	await invoke("plugin:conventions|function_with_no_weird_characters");
}
export async function apple() {
	await invoke("plugin:conventions|apple");
}
export async function applePear() {
	await invoke("plugin:conventions|apple_pear");
}
export async function applePearGrape() {
	await invoke("plugin:conventions|apple_pear_grape");
}
export async function a0() {
	await invoke("plugin:conventions|a0");
}
export async function isXml() {
	await invoke("plugin:conventions|is_xml");
}
export async function explicit() {
	await invoke("plugin:conventions|explicit");
}
export async function explicitKebab() {
	await invoke("plugin:conventions|explicit_kebab");
}
export async function bool() {
	await invoke("plugin:conventions|bool");
}

