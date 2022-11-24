const invoke = window.__TAURI_INVOKE__;
if (!window.__TAURI_BINDGEN_VERSION_CHECK__) {
  invoke("plugin|unions:cccf67b47414af61861a06498c06cf03").catch(() =>
    console.error(
      "The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`."
    )
  );
}

/**
 * @param {AllIntegers} num
 * @returns {Promise<AllIntegers>}
 */
export async function addOneInteger(num) {
  const result = await invoke("plugin:unions|add_one_integer", { num: num });
  return result;
}
/**
 * @param {AllFloats} num
 * @returns {Promise<AllFloats>}
 */
export async function addOneFloat(num) {
  const result = await invoke("plugin:unions|add_one_float", { num: num });
  return result;
}
/**
 * @param {AllText} text
 * @param {string} letter
 * @returns {Promise<AllText>}
 */
export async function replaceFirstChar(text, letter) {
  const result = await invoke("plugin:unions|replace_first_char", {
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
  const result = await invoke("plugin:unions|identify_integer", { num: num });
  return result;
}
/**
 * @param {AllFloats} num
 * @returns {Promise<number>}
 */
export async function identifyFloat(num) {
  const result = await invoke("plugin:unions|identify_float", { num: num });
  return result;
}
/**
 * @param {AllText} text
 * @returns {Promise<number>}
 */
export async function identifyText(text) {
  const result = await invoke("plugin:unions|identify_text", { text: text });
  return result;
}
/**
 * @param {DuplicatedS32} num
 * @returns {Promise<DuplicatedS32>}
 */
export async function addOneDuplicated(num) {
  const result = await invoke("plugin:unions|add_one_duplicated", { num: num });
  return result;
}
/**
 * @param {DuplicatedS32} num
 * @returns {Promise<number>}
 */
export async function identifyDuplicated(num) {
  const result = await invoke("plugin:unions|identify_duplicated", {
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
    num: num,
  });
  return result;
}
