const invoke = window.__TAURI_INVOKE__;
/**
 * @param {string} x
 * @returns {Promise<[]>}
 */
export async function a(x) {
	const result = await invoke("plugin:4883b53925a5f618|a", { x: x });
	return result;
}
/**
 * @returns {Promise<string>}
 */
export async function b() {
	const result = await invoke("plugin:4883b53925a5f618|b");
	return result;
}
/**
 * @param {string} a
 * @param {string} b
 * @returns {Promise<string>}
 */
export async function c(a, b) {
	const result = await invoke("plugin:4883b53925a5f618|c", { a: a, b: b });
	return result;
}

