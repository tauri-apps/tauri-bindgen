const invoke = window.__TAURI_INVOKE__;
if (!window.__TAURI_BINDGEN_VERSION_CHECK__) {
	invoke("plugin|simple_lists:d40a3203ef48115d7df3e6859a69ed77").catch(() =>
		console.error(
			"The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`.",
		),
	);
}

/**
 * @param {Uint32Array} l
 */
export async function simpleList1(l) {
	await invoke("plugin:simple_lists|simple_list1", { l: l });
}
/**
 * @returns {Promise<Uint32Array>}
 */
export async function simpleList2() {
	const result = await invoke("plugin:simple_lists|simple_list2");
	return result;
}
/**
 * @param {Uint32Array[]} l
 * @returns {Promise<Uint32Array[]>}
 */
export async function simpleList4(l) {
	const result = await invoke("plugin:simple_lists|simple_list4", { l: l });
	return result;
}

