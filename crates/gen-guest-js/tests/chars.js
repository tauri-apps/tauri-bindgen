const invoke = window.__TAURI_INVOKE__;
/**
 * A function that accepts a character
 * @param {string} x
 * @returns {Promise<[]>}
 */
export async function takeChar(x) {
	const result = await invoke("plugin:58d944fc9a2c8431|take_char", { x: x });
	return result;
}
/**
 * A function that returns a character
 * @returns {Promise<string>}
 */
export async function returnChar() {
	const result = await invoke("plugin:58d944fc9a2c8431|return_char");
	return result;
}

