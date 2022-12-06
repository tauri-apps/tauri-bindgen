const invoke = window.__TAURI_INVOKE__;
/**
 * @returns {Promise<Result<string | null, Error>>}
 */
export async function optionTest() {
	const result = await invoke("plugin:9f005dd416978e86|option_test");
	return result;
}

