const invoke = window.__TAURI_INVOKE__;
/**
 * @param {number} x
 */
export async function float32Param(x) {
	await invoke("plugin:b2ded0ef|float32_param", { x: x });
}
/**
 * @param {number} x
 */
export async function float64Param(x) {
	await invoke("plugin:b2ded0ef|float64_param", { x: x });
}
/**
 * @returns {Promise<number>}
 */
export async function float32Result() {
	const result = await invoke("plugin:b2ded0ef|float32_result");
	return result;
}
/**
 * @returns {Promise<number>}
 */
export async function float64Result() {
	const result = await invoke("plugin:b2ded0ef|float64_result");
	return result;
}

