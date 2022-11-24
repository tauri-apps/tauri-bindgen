const invoke = window.__TAURI_INVOKE__;
const idlHash = "cccf67b47414af61";
/**
 * @param {AllIntegers} num
 * @returns {Promise<AllIntegers>}
 */
export async function addOneInteger(num) {
  const result = await invoke("plugin:unions|add_one_integer", {
    idlHash,
    num: num,
  });
  return result;
}
/**
 * @param {AllFloats} num
 * @returns {Promise<AllFloats>}
 */
export async function addOneFloat(num) {
  const result = await invoke("plugin:unions|add_one_float", {
    idlHash,
    num: num,
  });
  return result;
}
/**
 * @param {AllText} text
 * @param {string} letter
 * @returns {Promise<AllText>}
 */
export async function replaceFirstChar(text, letter) {
  const result = await invoke("plugin:unions|replace_first_char", {
    idlHash,
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
  const result = await invoke("plugin:unions|identify_integer", {
    idlHash,
    num: num,
  });
  return result;
}
/**
 * @param {AllFloats} num
 * @returns {Promise<number>}
 */
export async function identifyFloat(num) {
  const result = await invoke("plugin:unions|identify_float", {
    idlHash,
    num: num,
  });
  return result;
}
/**
 * @param {AllText} text
 * @returns {Promise<number>}
 */
export async function identifyText(text) {
  const result = await invoke("plugin:unions|identify_text", {
    idlHash,
    text: text,
  });
  return result;
}
/**
 * @param {DuplicatedS32} num
 * @returns {Promise<DuplicatedS32>}
 */
export async function addOneDuplicated(num) {
  const result = await invoke("plugin:unions|add_one_duplicated", {
    idlHash,
    num: num,
  });
  return result;
}
/**
 * @param {DuplicatedS32} num
 * @returns {Promise<number>}
 */
export async function identifyDuplicated(num) {
  const result = await invoke("plugin:unions|identify_duplicated", {
    idlHash,
    num: num,
  });
  return result;
}
/**
 * @param {DistinguishableNum} num
 * @returns {Promise<DistinguishableNum>}
 */
export async function addOneDistinguishableNum(num) {
  const result = await invoke("plugin:unions|add_one_distinguishable_num", {
    idlHash,
    num: num,
  });
  return result;
}
/**
 * @param {DistinguishableNum} num
 * @returns {Promise<number>}
 */
export async function identifyDistinguishableNum(num) {
  const result = await invoke("plugin:unions|identify_distinguishable_num", {
    idlHash,
    num: num,
  });
  return result;
}
