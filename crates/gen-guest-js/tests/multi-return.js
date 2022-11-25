const invoke = window.__TAURI_INVOKE__;
export async function mra() {
	await invoke("plugin:d238f57052cdcb90|mra");
}
export async function mrb() {
	await invoke("plugin:d238f57052cdcb90|mrb");
}
/**
 * @returns {Promise<number>}
 */
export async function mrc() {
	const result = await invoke("plugin:d238f57052cdcb90|mrc");
	return result;
}
/**
 * @returns {Promise<number>}
 */
export async function mrd() {
	const result = await invoke("plugin:d238f57052cdcb90|mrd");
	return result;
}
/**
 * @returns {Promise<[number, number]>}
 */
export async function mre() {
	const result = await invoke("plugin:d238f57052cdcb90|mre");
	return result;
}

