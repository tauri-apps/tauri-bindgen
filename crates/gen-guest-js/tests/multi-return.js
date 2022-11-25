const invoke = window.__TAURI_INVOKE__;
if (!window.__TAURI_BINDGEN_VERSION_CHECK__) {
	invoke("plugin|multi_return:d238f57052cdcb9073d14f7a8059345b").catch(() =>
		console.error(
			"The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`.",
		),
	);
}

export async function mra() {
	await invoke("plugin:multi_return|mra");
}
export async function mrb() {
	await invoke("plugin:multi_return|mrb");
}
/**
 * @returns {Promise<number>}
 */
export async function mrc() {
	const result = await invoke("plugin:multi_return|mrc");
	return result;
}
/**
 * @returns {Promise<number>}
 */
export async function mrd() {
	const result = await invoke("plugin:multi_return|mrd");
	return result;
}
/**
 * @returns {Promise<[number, number]>}
 */
export async function mre() {
	const result = await invoke("plugin:multi_return|mre");
	return result;
}

