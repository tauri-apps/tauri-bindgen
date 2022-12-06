const invoke = window.__TAURI_INVOKE__;
/**
 * @param {E1} x
 * @returns {Promise<[]>}
 */
export async function e1Arg(x) {
	const result = await invoke("plugin:8178d1f91285bbc1|e1_arg", { x: x });
	return result;
}
/**
 * @returns {Promise<E1>}
 */
export async function e1Result() {
	const result = await invoke("plugin:8178d1f91285bbc1|e1_result");
	return result;
}
/**
 * @param {U1} x
 * @returns {Promise<[]>}
 */
export async function u1Arg(x) {
	const result = await invoke("plugin:8178d1f91285bbc1|u1_arg", { x: x });
	return result;
}
/**
 * @returns {Promise<U1>}
 */
export async function u1Result() {
	const result = await invoke("plugin:8178d1f91285bbc1|u1_result");
	return result;
}
/**
 * @param {V1} x
 * @returns {Promise<[]>}
 */
export async function v1Arg(x) {
	const result = await invoke("plugin:8178d1f91285bbc1|v1_arg", { x: x });
	return result;
}
/**
 * @returns {Promise<V1>}
 */
export async function v1Result() {
	const result = await invoke("plugin:8178d1f91285bbc1|v1_result");
	return result;
}
/**
 * @param {boolean} x
 * @returns {Promise<[]>}
 */
export async function boolArg(x) {
	const result = await invoke("plugin:8178d1f91285bbc1|bool_arg", { x: x });
	return result;
}
/**
 * @returns {Promise<boolean>}
 */
export async function boolResult() {
	const result = await invoke("plugin:8178d1f91285bbc1|bool_result");
	return result;
}
/**
 * @param {boolean | null} a
 * @param {[] | null} b
 * @param {number | null} c
 * @param {E1 | null} d
 * @param {number | null} e
 * @param {U1 | null} f
 * @param {boolean | null | null} g
 * @returns {Promise<[]>}
 */
export async function optionArg(a, b, c, d, e, f, g) {
	const result = await invoke("plugin:8178d1f91285bbc1|option_arg", {
		a: a,
		b: b,
		c: c,
		d: d,
		e: e,
		f: f,
		g: g,
	});
	return result;
}
/**
 * @returns {Promise<[boolean | null, [] | null, number | null, E1 | null, number | null, U1 | null, boolean | null | null]>}
 */
export async function optionResult() {
	const result = await invoke("plugin:8178d1f91285bbc1|option_result");
	return result;
}
/**
 * @param {Casts1} a
 * @param {Casts2} b
 * @param {Casts3} c
 * @param {Casts4} d
 * @param {Casts5} e
 * @param {Casts6} f
 * @returns {Promise<[Casts1, Casts2, Casts3, Casts4, Casts5, Casts6]>}
 */
export async function casts(a, b, c, d, e, f) {
	const result = await invoke("plugin:8178d1f91285bbc1|casts", {
		a: a,
		b: b,
		c: c,
		d: d,
		e: e,
		f: f,
	});
	return result;
}
/**
 * @param {Result<void, void>} a
 * @param {Result<void, E1>} b
 * @param {Result<E1, void>} c
 * @param {Result<[], []>} d
 * @param {Result<number, V1>} e
 * @param {Result<string, Uint8Array>} f
 * @returns {Promise<[]>}
 */
export async function resultArg(a, b, c, d, e, f) {
	const result = await invoke("plugin:8178d1f91285bbc1|result_arg", {
		a: a,
		b: b,
		c: c,
		d: d,
		e: e,
		f: f,
	});
	return result;
}
/**
 * @returns {Promise<[Result<void, void>, Result<void, E1>, Result<E1, void>, Result<[], []>, Result<number, V1>, Result<string, Uint8Array>]>}
 */
export async function resultResult() {
	const result = await invoke("plugin:8178d1f91285bbc1|result_result");
	return result;
}
/**
 * @returns {Promise<Result<number, MyErrno>>}
 */
export async function returnResultSugar() {
	const result = await invoke("plugin:8178d1f91285bbc1|return_result_sugar");
	return result;
}
/**
 * @returns {Promise<Result<void, MyErrno>>}
 */
export async function returnResultSugar2() {
	const result = await invoke("plugin:8178d1f91285bbc1|return_result_sugar2");
	return result;
}
/**
 * @returns {Promise<Result<MyErrno, MyErrno>>}
 */
export async function returnResultSugar3() {
	const result = await invoke("plugin:8178d1f91285bbc1|return_result_sugar3");
	return result;
}
/**
 * @returns {Promise<Result<[number, number], MyErrno>>}
 */
export async function returnResultSugar4() {
	const result = await invoke("plugin:8178d1f91285bbc1|return_result_sugar4");
	return result;
}
/**
 * @returns {Promise<number | null>}
 */
export async function returnOptionSugar() {
	const result = await invoke("plugin:8178d1f91285bbc1|return_option_sugar");
	return result;
}
/**
 * @returns {Promise<MyErrno | null>}
 */
export async function returnOptionSugar2() {
	const result = await invoke("plugin:8178d1f91285bbc1|return_option_sugar2");
	return result;
}
/**
 * @returns {Promise<Result<number, number>>}
 */
export async function resultSimple() {
	const result = await invoke("plugin:8178d1f91285bbc1|result_simple");
	return result;
}
/**
 * @param {IsClone} a
 * @returns {Promise<[]>}
 */
export async function isCloneArg(a) {
	const result = await invoke("plugin:8178d1f91285bbc1|is_clone_arg", { a: a });
	return result;
}
/**
 * @returns {Promise<IsClone>}
 */
export async function isCloneReturn() {
	const result = await invoke("plugin:8178d1f91285bbc1|is_clone_return");
	return result;
}
/**
 * @returns {Promise<number | null>}
 */
export async function returnNamedOption() {
	const result = await invoke("plugin:8178d1f91285bbc1|return_named_option");
	return result;
}
/**
 * @returns {Promise<Result<number, MyErrno>>}
 */
export async function returnNamedResult() {
	const result = await invoke("plugin:8178d1f91285bbc1|return_named_result");
	return result;
}

