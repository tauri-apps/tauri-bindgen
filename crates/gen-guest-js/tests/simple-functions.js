const invoke = window.__TAURI_INVOKE__;
/**
 * @returns {Promise<[]>}
 */
export async function f1() {
	const result = await invoke("plugin:d52f0e93c1bb4daa|f1");
	return result;
}
/**
 * @param {number} a
 * @returns {Promise<[]>}
 */
export async function f2(a) {
	const result = await invoke("plugin:d52f0e93c1bb4daa|f2", { a: a });
	return result;
}
/**
 * @param {number} a
 * @param {number} b
 * @returns {Promise<[]>}
 */
export async function f3(a, b) {
	const result = await invoke("plugin:d52f0e93c1bb4daa|f3", { a: a, b: b });
	return result;
}
/**
 * @returns {Promise<number>}
 */
export async function f4() {
	const result = await invoke("plugin:d52f0e93c1bb4daa|f4");
	return result;
}
/**
 * @returns {Promise<[number, number]>}
 */
export async function f5() {
	const result = await invoke("plugin:d52f0e93c1bb4daa|f5");
	return result;
}
/**
 * @param {number} a
 * @param {number} b
 * @param {number} c
 * @returns {Promise<[number, number, number]>}
 */
export async function f6(a, b, c) {
	const result = await invoke("plugin:d52f0e93c1bb4daa|f6", {
		a: a,
		b: b,
		c: c,
	});
	return result;
}

