const invoke = window.__TAURI_INVOKE__;
/**
 * @param {[string, number]} x
 */
export async function tupleArg(x) {
	await invoke("plugin:e6872cf01241a6f3|tuple_arg", { x: x });
}
/**
 * @returns {Promise<[string, number]>}
 */
export async function tupleResult() {
	const result = await invoke("plugin:e6872cf01241a6f3|tuple_result");
	return result;
}
/**
 * @param {Empty} x
 */
export async function emptyArg(x) {
	await invoke("plugin:e6872cf01241a6f3|empty_arg", { x: x });
}
/**
 * @returns {Promise<Empty>}
 */
export async function emptyResult() {
	const result = await invoke("plugin:e6872cf01241a6f3|empty_result");
	return result;
}
/**
 * @param {Scalars} x
 */
export async function scalarArg(x) {
	await invoke("plugin:e6872cf01241a6f3|scalar_arg", { x: x });
}
/**
 * @returns {Promise<Scalars>}
 */
export async function scalarResult() {
	const result = await invoke("plugin:e6872cf01241a6f3|scalar_result");
	return result;
}
/**
 * @param {ReallyFlags} x
 */
export async function flagsArg(x) {
	await invoke("plugin:e6872cf01241a6f3|flags_arg", { x: x });
}
/**
 * @returns {Promise<ReallyFlags>}
 */
export async function flagsResult() {
	const result = await invoke("plugin:e6872cf01241a6f3|flags_result");
	return result;
}
/**
 * @param {Aggregates} x
 */
export async function aggregateArg(x) {
	await invoke("plugin:e6872cf01241a6f3|aggregate_arg", { x: x });
}
/**
 * @returns {Promise<Aggregates>}
 */
export async function aggregateResult() {
	const result = await invoke("plugin:e6872cf01241a6f3|aggregate_result");
	return result;
}
/**
 * @param {TupleTypedef2} e
 * @returns {Promise<number>}
 */
export async function typedefInout(e) {
	const result = await invoke("plugin:e6872cf01241a6f3|typedef_inout", {
		e: e,
	});
	return result;
}

