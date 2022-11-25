const invoke = window.__TAURI_INVOKE__;
/**
 * @param {AllIntegers} num
 * @returns {Promise<AllIntegers>}
 */
export async function addOneInteger(num) {
	const result = await invoke("plugin:cccf67b4|add_one_integer", { num: num });
	return result;
}
/**
 * @param {AllFloats} num
 * @returns {Promise<AllFloats>}
 */
export async function addOneFloat(num) {
	const result = await invoke("plugin:cccf67b4|add_one_float", { num: num });
	return result;
}
/**
 * @param {AllText} text
 * @param {string} letter
 * @returns {Promise<AllText>}
 */
export async function replaceFirstChar(text, letter) {
	const result = await invoke("plugin:cccf67b4|replace_first_char", {
		text: text,
		letter: letter,
	});
	return result;
}
/**
 * @param {AllIntegers} num
 * @returns {Promise<number>}
 */
export async function identifyInteger(num) {
	const result = await invoke("plugin:cccf67b4|identify_integer", { num: num });
	return result;
}
/**
 * @param {AllFloats} num
 * @returns {Promise<number>}
 */
export async function identifyFloat(num) {
	const result = await invoke("plugin:cccf67b4|identify_float", { num: num });
	return result;
}
/**
 * @param {AllText} text
 * @returns {Promise<number>}
 */
export async function identifyText(text) {
	const result = await invoke("plugin:cccf67b4|identify_text", { text: text });
	return result;
}
/**
 * @param {DuplicatedS32} num
 * @returns {Promise<DuplicatedS32>}
 */
export async function addOneDuplicated(num) {
	const result = await invoke("plugin:cccf67b4|add_one_duplicated", {
		num: num,
	});
	return result;
}
/**
 * @param {DuplicatedS32} num
 * @returns {Promise<number>}
 */
export async function identifyDuplicated(num) {
	const result = await invoke("plugin:cccf67b4|identify_duplicated", {
		num: num,
	});
	return result;
}
/**
 * @param {DistinguishableNum} num
 * @returns {Promise<DistinguishableNum>}
 */
export async function addOneDistinguishableNum(num) {
	const result = await invoke("plugin:cccf67b4|add_one_distinguishable_num", {
		num: num,
	});
	return result;
}
/**
 * @param {DistinguishableNum} num
 * @returns {Promise<number>}
 */
export async function identifyDistinguishableNum(num) {
	const result = await invoke("plugin:cccf67b4|identify_distinguishable_num", {
		num: num,
	});
	return result;
}

