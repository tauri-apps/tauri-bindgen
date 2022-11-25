const invoke = window.__TAURI_INVOKE__;
export async function f1() {
	await invoke("plugin:ebb2d6f0|f1");
}
/**
 * @param {number} a
 */
export async function f2(a) {
	await invoke("plugin:ebb2d6f0|f2", { a: a });
}
/**
 * @param {number} a
 * @param {number} b
 */
export async function f3(a, b) {
	await invoke("plugin:ebb2d6f0|f3", { a: a, b: b });
}
/**
 * @returns {Promise<number>}
 */
export async function f4() {
	const result = await invoke("plugin:ebb2d6f0|f4");
	return result;
}
/**
 * @returns {Promise<[number, number]>}
 */
export async function f5() {
	const result = await invoke("plugin:ebb2d6f0|f5");
	return result;
}
/**
 * @param {number} a
 * @param {number} b
 * @param {number} c
 * @returns {Promise<[number, number, number]>}
 */
export async function f6(a, b, c) {
	const result = await invoke("plugin:ebb2d6f0|f6", { a: a, b: b, c: c });
	return result;
}

