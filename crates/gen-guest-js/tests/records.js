const invoke = window.__TAURI_INVOKE__;
const idlHash = "e6872cf01241a6f3";
/**
 * @param {[string, number]} x
 */
export async function tupleArg(x) {
  await invoke("plugin:records|tuple_arg", { idlHash, x: x });
}
/**
 * @returns {Promise<[string, number]>}
 */
export async function tupleResult() {
  const result = await invoke("plugin:records|tuple_result", { idlHash });
  return result;
}
/**
 * @param {Empty} x
 */
export async function emptyArg(x) {
  await invoke("plugin:records|empty_arg", { idlHash, x: x });
}
/**
 * @returns {Promise<Empty>}
 */
export async function emptyResult() {
  const result = await invoke("plugin:records|empty_result", { idlHash });
  return result;
}
/**
 * @param {Scalars} x
 */
export async function scalarArg(x) {
  await invoke("plugin:records|scalar_arg", { idlHash, x: x });
}
/**
 * @returns {Promise<Scalars>}
 */
export async function scalarResult() {
  const result = await invoke("plugin:records|scalar_result", { idlHash });
  return result;
}
/**
 * @param {ReallyFlags} x
 */
export async function flagsArg(x) {
  await invoke("plugin:records|flags_arg", { idlHash, x: x });
}
/**
 * @returns {Promise<ReallyFlags>}
 */
export async function flagsResult() {
  const result = await invoke("plugin:records|flags_result", { idlHash });
  return result;
}
/**
 * @param {Aggregates} x
 */
export async function aggregateArg(x) {
  await invoke("plugin:records|aggregate_arg", { idlHash, x: x });
}
/**
 * @returns {Promise<Aggregates>}
 */
export async function aggregateResult() {
  const result = await invoke("plugin:records|aggregate_result", { idlHash });
  return result;
}
/**
 * @param {TupleTypedef2} e
 * @returns {Promise<number>}
 */
export async function typedefInout(e) {
  const result = await invoke("plugin:records|typedef_inout", {
    idlHash,
    e: e,
  });
  return result;
}
