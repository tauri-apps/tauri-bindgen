const invoke = window.__TAURI_INVOKE__;
/**
 * @returns {Promise<string | null>}
 */
export async function optionTest() {
	const result = await invoke("plugin:bee731db|option_test");
	return result;
}

