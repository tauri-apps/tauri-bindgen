const invoke = window.__TAURI_INVOKE__;
/**
 * @param {Flag1} x
 * @returns {Promise<Flag1>}
 */
export async function roundtripFlag1(x) {
	const result = await invoke("plugin:8ecd22d5a53ba1eb|roundtrip_flag1", {
		x: x,
	});
	return result;
}
/**
 * @param {Flag2} x
 * @returns {Promise<Flag2>}
 */
export async function roundtripFlag2(x) {
	const result = await invoke("plugin:8ecd22d5a53ba1eb|roundtrip_flag2", {
		x: x,
	});
	return result;
}
/**
 * @param {Flag4} x
 * @returns {Promise<Flag4>}
 */
export async function roundtripFlag4(x) {
	const result = await invoke("plugin:8ecd22d5a53ba1eb|roundtrip_flag4", {
		x: x,
	});
	return result;
}
/**
 * @param {Flag8} x
 * @returns {Promise<Flag8>}
 */
export async function roundtripFlag8(x) {
	const result = await invoke("plugin:8ecd22d5a53ba1eb|roundtrip_flag8", {
		x: x,
	});
	return result;
}
/**
 * @param {Flag16} x
 * @returns {Promise<Flag16>}
 */
export async function roundtripFlag16(x) {
	const result = await invoke("plugin:8ecd22d5a53ba1eb|roundtrip_flag16", {
		x: x,
	});
	return result;
}
/**
 * @param {Flag32} x
 * @returns {Promise<Flag32>}
 */
export async function roundtripFlag32(x) {
	const result = await invoke("plugin:8ecd22d5a53ba1eb|roundtrip_flag32", {
		x: x,
	});
	return result;
}
/**
 * @param {Flag64} x
 * @returns {Promise<Flag64>}
 */
export async function roundtripFlag64(x) {
	const result = await invoke("plugin:8ecd22d5a53ba1eb|roundtrip_flag64", {
		x: x,
	});
	return result;
}

