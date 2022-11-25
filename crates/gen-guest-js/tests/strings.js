const invoke = window.__TAURI_INVOKE__;
/**
 * @param {string} x
 */
export async function a(x) {
	await invoke("plugin:16c3ebd2deefea81|a", { x: x });
}
/**
 * @returns {Promise<string>}
 */
export async function b() {
	const result = await invoke("plugin:16c3ebd2deefea81|b");
	return result;
}
/**
 * @param {string} a
 * @param {string} b
 * @returns {Promise<string>}
 */
export async function c(a, b) {
	const result = await invoke("plugin:16c3ebd2deefea81|c", { a: a, b: b });
	return result;
}

