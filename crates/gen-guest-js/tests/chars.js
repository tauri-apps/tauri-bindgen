const invoke = window.__TAURI_INVOKE__;
/**
 * A function that accepts a character
 * @param {string} x
 */
export async function takeChar(x) {
	await invoke("plugin:678374cfb5cdb2b5|take_char", { x: x });
}
/**
 * A function that returns a character
 * @returns {Promise<string>}
 */
export async function returnChar() {
	const result = await invoke("plugin:678374cfb5cdb2b5|return_char");
	return result;
}

