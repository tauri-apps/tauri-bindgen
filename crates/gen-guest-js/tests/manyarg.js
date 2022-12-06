const invoke = window.__TAURI_INVOKE__;
/**
 * @param {bigint} a1
 * @param {bigint} a2
 * @param {bigint} a3
 * @param {bigint} a4
 * @param {bigint} a5
 * @param {bigint} a6
 * @param {bigint} a7
 * @param {bigint} a8
 * @param {bigint} a9
 * @param {bigint} a10
 * @param {bigint} a11
 * @param {bigint} a12
 * @param {bigint} a13
 * @param {bigint} a14
 * @param {bigint} a15
 * @param {bigint} a16
 * @returns {Promise<[]>}
 */
export async function manyArgs(
	a1,
	a2,
	a3,
	a4,
	a5,
	a6,
	a7,
	a8,
	a9,
	a10,
	a11,
	a12,
	a13,
	a14,
	a15,
	a16,
) {
	const result = await invoke("plugin:0dadf5dbb1f5700d|many_args", {
		a1: a1,
		a2: a2,
		a3: a3,
		a4: a4,
		a5: a5,
		a6: a6,
		a7: a7,
		a8: a8,
		a9: a9,
		a10: a10,
		a11: a11,
		a12: a12,
		a13: a13,
		a14: a14,
		a15: a15,
		a16: a16,
	});
	return result;
}
/**
 * @param {BigStruct} x
 * @returns {Promise<[]>}
 */
export async function bigArgument(x) {
	const result = await invoke("plugin:0dadf5dbb1f5700d|big_argument", { x: x });
	return result;
}

