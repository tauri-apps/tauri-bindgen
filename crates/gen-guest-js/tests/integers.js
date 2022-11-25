const invoke = window.__TAURI_INVOKE__;
/**
 * @param {number} x
 */
export async function a1(x) {
	await invoke("plugin:279b557e344c2e05|a1", { x: x });
}
/**
 * @param {number} x
 */
export async function a2(x) {
	await invoke("plugin:279b557e344c2e05|a2", { x: x });
}
/**
 * @param {number} x
 */
export async function a3(x) {
	await invoke("plugin:279b557e344c2e05|a3", { x: x });
}
/**
 * @param {number} x
 */
export async function a4(x) {
	await invoke("plugin:279b557e344c2e05|a4", { x: x });
}
/**
 * @param {number} x
 */
export async function a5(x) {
	await invoke("plugin:279b557e344c2e05|a5", { x: x });
}
/**
 * @param {number} x
 */
export async function a6(x) {
	await invoke("plugin:279b557e344c2e05|a6", { x: x });
}
/**
 * @param {bigint} x
 */
export async function a7(x) {
	await invoke("plugin:279b557e344c2e05|a7", { x: x });
}
/**
 * @param {bigint} x
 */
export async function a8(x) {
	await invoke("plugin:279b557e344c2e05|a8", { x: x });
}
/**
 * @param {number} p1
 * @param {number} p2
 * @param {number} p3
 * @param {number} p4
 * @param {number} p5
 * @param {number} p6
 * @param {bigint} p7
 * @param {bigint} p8
 */
export async function a9(p1, p2, p3, p4, p5, p6, p7, p8) {
	await invoke("plugin:279b557e344c2e05|a9", {
		p1: p1,
		p2: p2,
		p3: p3,
		p4: p4,
		p5: p5,
		p6: p6,
		p7: p7,
		p8: p8,
	});
}
/**
 * @returns {Promise<number>}
 */
export async function r1() {
	const result = await invoke("plugin:279b557e344c2e05|r1");
	return result;
}
/**
 * @returns {Promise<number>}
 */
export async function r2() {
	const result = await invoke("plugin:279b557e344c2e05|r2");
	return result;
}
/**
 * @returns {Promise<number>}
 */
export async function r3() {
	const result = await invoke("plugin:279b557e344c2e05|r3");
	return result;
}
/**
 * @returns {Promise<number>}
 */
export async function r4() {
	const result = await invoke("plugin:279b557e344c2e05|r4");
	return result;
}
/**
 * @returns {Promise<number>}
 */
export async function r5() {
	const result = await invoke("plugin:279b557e344c2e05|r5");
	return result;
}
/**
 * @returns {Promise<number>}
 */
export async function r6() {
	const result = await invoke("plugin:279b557e344c2e05|r6");
	return result;
}
/**
 * @returns {Promise<bigint>}
 */
export async function r7() {
	const result = await invoke("plugin:279b557e344c2e05|r7");
	return result;
}
/**
 * @returns {Promise<bigint>}
 */
export async function r8() {
	const result = await invoke("plugin:279b557e344c2e05|r8");
	return result;
}
/**
 * @returns {Promise<[bigint, number]>}
 */
export async function pairRet() {
	const result = await invoke("plugin:279b557e344c2e05|pair_ret");
	return result;
}

