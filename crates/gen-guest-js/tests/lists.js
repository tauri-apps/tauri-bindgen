const invoke = window.__TAURI_INVOKE__;
/**
 * @param {Uint8Array} x
 */
export async function listU8Param(x) {
	await invoke("plugin:a744d1c6fec40184|list_u8_param", { x: x });
}
/**
 * @param {Uint16Array} x
 */
export async function listU16Param(x) {
	await invoke("plugin:a744d1c6fec40184|list_u16_param", { x: x });
}
/**
 * @param {Uint32Array} x
 */
export async function listU32Param(x) {
	await invoke("plugin:a744d1c6fec40184|list_u32_param", { x: x });
}
/**
 * @param {BigUint64Array} x
 */
export async function listU64Param(x) {
	await invoke("plugin:a744d1c6fec40184|list_u64_param", { x: x });
}
/**
 * @param {Int8Array} x
 */
export async function listS8Param(x) {
	await invoke("plugin:a744d1c6fec40184|list_s8_param", { x: x });
}
/**
 * @param {Int16Array} x
 */
export async function listS16Param(x) {
	await invoke("plugin:a744d1c6fec40184|list_s16_param", { x: x });
}
/**
 * @param {Int32Array} x
 */
export async function listS32Param(x) {
	await invoke("plugin:a744d1c6fec40184|list_s32_param", { x: x });
}
/**
 * @param {BigInt64Array} x
 */
export async function listS64Param(x) {
	await invoke("plugin:a744d1c6fec40184|list_s64_param", { x: x });
}
/**
 * @param {Float32Array} x
 */
export async function listFloat32Param(x) {
	await invoke("plugin:a744d1c6fec40184|list_float32_param", { x: x });
}
/**
 * @param {Float64Array} x
 */
export async function listFloat64Param(x) {
	await invoke("plugin:a744d1c6fec40184|list_float64_param", { x: x });
}
/**
 * @returns {Promise<Uint8Array>}
 */
export async function listU8Ret() {
	const result = await invoke("plugin:a744d1c6fec40184|list_u8_ret");
	return result;
}
/**
 * @returns {Promise<Uint16Array>}
 */
export async function listU16Ret() {
	const result = await invoke("plugin:a744d1c6fec40184|list_u16_ret");
	return result;
}
/**
 * @returns {Promise<Uint32Array>}
 */
export async function listU32Ret() {
	const result = await invoke("plugin:a744d1c6fec40184|list_u32_ret");
	return result;
}
/**
 * @returns {Promise<BigUint64Array>}
 */
export async function listU64Ret() {
	const result = await invoke("plugin:a744d1c6fec40184|list_u64_ret");
	return result;
}
/**
 * @returns {Promise<Int8Array>}
 */
export async function listS8Ret() {
	const result = await invoke("plugin:a744d1c6fec40184|list_s8_ret");
	return result;
}
/**
 * @returns {Promise<Int16Array>}
 */
export async function listS16Ret() {
	const result = await invoke("plugin:a744d1c6fec40184|list_s16_ret");
	return result;
}
/**
 * @returns {Promise<Int32Array>}
 */
export async function listS32Ret() {
	const result = await invoke("plugin:a744d1c6fec40184|list_s32_ret");
	return result;
}
/**
 * @returns {Promise<BigInt64Array>}
 */
export async function listS64Ret() {
	const result = await invoke("plugin:a744d1c6fec40184|list_s64_ret");
	return result;
}
/**
 * @returns {Promise<Float32Array>}
 */
export async function listFloat32Ret() {
	const result = await invoke("plugin:a744d1c6fec40184|list_float32_ret");
	return result;
}
/**
 * @returns {Promise<Float64Array>}
 */
export async function listFloat64Ret() {
	const result = await invoke("plugin:a744d1c6fec40184|list_float64_ret");
	return result;
}
/**
 * @param {[number, number][]} x
 * @returns {Promise<[bigint, number][]>}
 */
export async function tupleList(x) {
	const result = await invoke("plugin:a744d1c6fec40184|tuple_list", { x: x });
	return result;
}
/**
 * @param {string[]} a
 */
export async function stringListArg(a) {
	await invoke("plugin:a744d1c6fec40184|string_list_arg", { a: a });
}
/**
 * @returns {Promise<string[]>}
 */
export async function stringListRet() {
	const result = await invoke("plugin:a744d1c6fec40184|string_list_ret");
	return result;
}
/**
 * @param {[number, string][]} x
 * @returns {Promise<[string, number][]>}
 */
export async function tupleStringList(x) {
	const result = await invoke("plugin:a744d1c6fec40184|tuple_string_list", {
		x: x,
	});
	return result;
}
/**
 * @param {string[]} x
 * @returns {Promise<string[]>}
 */
export async function stringList(x) {
	const result = await invoke("plugin:a744d1c6fec40184|string_list", { x: x });
	return result;
}
/**
 * @param {SomeRecord[]} x
 * @returns {Promise<OtherRecord[]>}
 */
export async function recordList(x) {
	const result = await invoke("plugin:a744d1c6fec40184|record_list", { x: x });
	return result;
}
/**
 * @param {OtherRecord[]} x
 * @returns {Promise<SomeRecord[]>}
 */
export async function recordListReverse(x) {
	const result = await invoke("plugin:a744d1c6fec40184|record_list_reverse", {
		x: x,
	});
	return result;
}
/**
 * @param {SomeVariant[]} x
 * @returns {Promise<OtherVariant[]>}
 */
export async function variantList(x) {
	const result = await invoke("plugin:a744d1c6fec40184|variant_list", { x: x });
	return result;
}
/**
 * @param {LoadStoreAllSizes} a
 * @returns {Promise<LoadStoreAllSizes>}
 */
export async function loadStoreEverything(a) {
	const result = await invoke("plugin:a744d1c6fec40184|load_store_everything", {
		a: a,
	});
	return result;
}

