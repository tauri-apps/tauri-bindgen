const invoke = window.__TAURI_INVOKE__;
/**
 * @returns {Promise<[]>}
 */
export async function kebabCase() {
	const result = await invoke("plugin:3d46778dded1f0fb|kebab_case");
	return result;
}
/**
 * @param {LudicrousSpeed} x
 * @returns {Promise<[]>}
 */
export async function foo(x) {
	const result = await invoke("plugin:3d46778dded1f0fb|foo", { x: x });
	return result;
}
/**
 * @returns {Promise<[]>}
 */
export async function functionWithDashes() {
	const result = await invoke("plugin:3d46778dded1f0fb|function_with_dashes");
	return result;
}
/**
 * @returns {Promise<[]>}
 */
export async function functionWithNoWeirdCharacters() {
	const result = await invoke(
		"plugin:3d46778dded1f0fb|function_with_no_weird_characters",
	);
	return result;
}
/**
 * @returns {Promise<[]>}
 */
export async function apple() {
	const result = await invoke("plugin:3d46778dded1f0fb|apple");
	return result;
}
/**
 * @returns {Promise<[]>}
 */
export async function applePear() {
	const result = await invoke("plugin:3d46778dded1f0fb|apple_pear");
	return result;
}
/**
 * @returns {Promise<[]>}
 */
export async function applePearGrape() {
	const result = await invoke("plugin:3d46778dded1f0fb|apple_pear_grape");
	return result;
}
/**
 * @returns {Promise<[]>}
 */
export async function a0() {
	const result = await invoke("plugin:3d46778dded1f0fb|a0");
	return result;
}
/**
 * @returns {Promise<[]>}
 */
export async function isXml() {
	const result = await invoke("plugin:3d46778dded1f0fb|is_xml");
	return result;
}
/**
 * @returns {Promise<[]>}
 */
export async function explicit() {
	const result = await invoke("plugin:3d46778dded1f0fb|explicit");
	return result;
}
/**
 * @returns {Promise<[]>}
 */
export async function explicitKebab() {
	const result = await invoke("plugin:3d46778dded1f0fb|explicit_kebab");
	return result;
}
/**
 * @returns {Promise<[]>}
 */
export async function bool() {
	const result = await invoke("plugin:3d46778dded1f0fb|bool");
	return result;
}

