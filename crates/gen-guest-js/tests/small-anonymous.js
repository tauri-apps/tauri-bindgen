const invoke = window.__TAURI_INVOKE__;
if (!window.__TAURI_BINDGEN_VERSION_CHECK__) {
	invoke("plugin|anon:bee731db80799df9a7eea6b7e0b7a0ce").catch(() =>
		console.error(
			"The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`.",
		),
	);
}

/**
 * @returns {Promise<string | null>}
 */
export async function optionTest() {
	const result = await invoke("plugin:anon|option_test");
	return result;
}

