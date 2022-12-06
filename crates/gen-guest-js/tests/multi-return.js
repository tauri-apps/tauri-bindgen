const invoke = window.__TAURI_INVOKE__;
/**
 * @returns {Promise<[]>}
 */
export async function mra() {
	const result = await invoke("plugin:def17a258c1e4f4d|mra");
	return result;
}
export async function mrb() {
	await invoke("plugin:def17a258c1e4f4d|mrb");
}
/**
 * @returns {Promise<number>}
 */
export async function mrc() {
	const result = await invoke("plugin:def17a258c1e4f4d|mrc");
	return result;
}
/**
 * @returns {Promise<number>}
 */
export async function mrd() {
	const result = await invoke("plugin:def17a258c1e4f4d|mrd");
	return result;
}
/**
 * @returns {Promise<[number, number]>}
 */
export async function mre() {
	const result = await invoke("plugin:def17a258c1e4f4d|mre");
	return result;
}

