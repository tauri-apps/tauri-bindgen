const { invoke } = window.__TAURI__.tauri;
/**
 * @param {[string, number]} x
 */
export async function tupleArg(x) {
  await invoke("plugin:imports|tuple_arg", { x: x });
}
/**
 * @returns {Promise<[string, number]>}
 */
export async function tupleResult() {
  const result = await invoke("plugin:imports|tuple_result");
  return result;
}
/**
 * @param {Empty} x
 */
export async function emptyArg(x) {
  await invoke("plugin:imports|empty_arg", { x: x });
}
/**
 * @returns {Promise<Empty>}
 */
export async function emptyResult() {
  const result = await invoke("plugin:imports|empty_result");
  return result;
}
/**
 * @param {Scalars} x
 */
export async function scalarArg(x) {
  await invoke("plugin:imports|scalar_arg", { x: x });
}
/**
 * @returns {Promise<Scalars>}
 */
export async function scalarResult() {
  const result = await invoke("plugin:imports|scalar_result");
  return result;
}
/**
 * @param {ReallyFlags} x
 */
export async function flagsArg(x) {
  await invoke("plugin:imports|flags_arg", { x: x });
}
/**
 * @returns {Promise<ReallyFlags>}
 */
export async function flagsResult() {
  const result = await invoke("plugin:imports|flags_result");
  return result;
}
/**
 * @param {Aggregates} x
 */
export async function aggregateArg(x) {
  await invoke("plugin:imports|aggregate_arg", { x: x });
}
/**
 * @returns {Promise<Aggregates>}
 */
export async function aggregateResult() {
  const result = await invoke("plugin:imports|aggregate_result");
  return result;
}
/**
 * @param {TupleTypedef2} e
 * @returns {Promise<number>}
 */
export async function typedefInout(e) {
  const result = await invoke("plugin:imports|typedef_inout", { e: e });
  return result;
}
