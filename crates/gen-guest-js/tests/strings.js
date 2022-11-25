const invoke = window.__TAURI_INVOKE__;
if (!window.__TAURI_BINDGEN_VERSION_CHECK__) {
	invoke("plugin|strings:16c3ebd2deefea81065e2001501951a6").catch(() =>
		console.error(
			"The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`.",
		),
	);
}

/**
 * @param {string} x
 */
export async function a(x) {
	await invoke("plugin:strings|a", { x: x });
}
/**
 * @returns {Promise<string>}
 */
export async function b() {
	const result = await invoke("plugin:strings|b");
	return result;
}
/**
 * @param {string} a
 * @param {string} b
 * @returns {Promise<string>}
 */
export async function c(a, b) {
	const result = await invoke("plugin:strings|c", { a: a, b: b });
	return result;
}

