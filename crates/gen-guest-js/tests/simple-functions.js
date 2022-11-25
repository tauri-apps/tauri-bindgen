const invoke = window.__TAURI_INVOKE__;
if (!window.__TAURI_BINDGEN_VERSION_CHECK__) {
	invoke("plugin|simple:ebb2d6f0441e00a02915e2faf10bbe90").catch(() =>
		console.error(
			"The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`.",
		),
	);
}

export async function f1() {
	await invoke("plugin:simple|f1");
}
/**
 * @param {number} a
 */
export async function f2(a) {
	await invoke("plugin:simple|f2", { a: a });
}
/**
 * @param {number} a
 * @param {number} b
 */
export async function f3(a, b) {
	await invoke("plugin:simple|f3", { a: a, b: b });
}
/**
 * @returns {Promise<number>}
 */
export async function f4() {
	const result = await invoke("plugin:simple|f4");
	return result;
}
/**
 * @returns {Promise<[number, number]>}
 */
export async function f5() {
	const result = await invoke("plugin:simple|f5");
	return result;
}
/**
 * @param {number} a
 * @param {number} b
 * @param {number} c
 * @returns {Promise<[number, number, number]>}
 */
export async function f6(a, b, c) {
	const result = await invoke("plugin:simple|f6", { a: a, b: b, c: c });
	return result;
}

