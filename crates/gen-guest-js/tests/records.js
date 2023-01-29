const invoke = window.__TAURI_INVOKE__;
/**
* @param {[string, number]} x
* @returns {Promise<[]>}
*/
export async function tupleArg(x) {
  const result = await invoke("plugin:ac98167d7d43eb21|tuple_arg",{x: x});
  return result
}
/**
* @returns {Promise<[string, number]>}
*/
export async function tupleResult() {
  const result = await invoke("plugin:ac98167d7d43eb21|tuple_result",);
  return result
}
/**
* @param {Empty} x
* @returns {Promise<[]>}
*/
export async function emptyArg(x) {
  const result = await invoke("plugin:ac98167d7d43eb21|empty_arg",{x: x});
  return result
}
/**
* @returns {Promise<Empty>}
*/
export async function emptyResult() {
  const result = await invoke("plugin:ac98167d7d43eb21|empty_result",);
  return result
}
/**
* @param {Scalars} x
* @returns {Promise<[]>}
*/
export async function scalarArg(x) {
  const result = await invoke("plugin:ac98167d7d43eb21|scalar_arg",{x: x});
  return result
}
/**
* @returns {Promise<Scalars>}
*/
export async function scalarResult() {
  const result = await invoke("plugin:ac98167d7d43eb21|scalar_result",);
  return result
}
/**
* @param {ReallyFlags} x
* @returns {Promise<[]>}
*/
export async function flagsArg(x) {
  const result = await invoke("plugin:ac98167d7d43eb21|flags_arg",{x: x});
  return result
}
/**
* @returns {Promise<ReallyFlags>}
*/
export async function flagsResult() {
  const result = await invoke("plugin:ac98167d7d43eb21|flags_result",);
  return result
}
/**
* @param {Aggregates} x
* @returns {Promise<[]>}
*/
export async function aggregateArg(x) {
  const result = await invoke("plugin:ac98167d7d43eb21|aggregate_arg",{x: x});
  return result
}
/**
* @returns {Promise<Aggregates>}
*/
export async function aggregateResult() {
  const result = await invoke("plugin:ac98167d7d43eb21|aggregate_result",);
  return result
}
/**
* @param {TupleTypedef2} e
* @returns {Promise<number>}
*/
export async function typedefInout(e) {
  const result = await invoke("plugin:ac98167d7d43eb21|typedef_inout",{e: e});
  return result
}

