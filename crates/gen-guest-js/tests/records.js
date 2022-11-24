const invoke = window.__TAURI_INVOKE__;
if (!window.__TAURI_BINDGEN_VERSION_CHECK__) {
  invoke("plugin|records:e6872cf01241a6f3e4c4bedaa609dbeb").catch(() =>
    console.error(
      "The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`."
    )
  );
}

/**
 * @param {[string, number]} x
 */
export async function tupleArg(x) {
  await invoke("plugin:records|tuple_arg", { x: x });
}
/**
 * @returns {Promise<[string, number]>}
 */
export async function tupleResult() {
  const result = await invoke("plugin:records|tuple_result");
  return result;
}
/**
 * @param {Empty} x
 */
export async function emptyArg(x) {
  await invoke("plugin:records|empty_arg", { x: x });
}
/**
 * @returns {Promise<Empty>}
 */
export async function emptyResult() {
  const result = await invoke("plugin:records|empty_result");
  return result;
}
/**
 * @param {Scalars} x
 */
export async function scalarArg(x) {
  await invoke("plugin:records|scalar_arg", { x: x });
}
/**
 * @returns {Promise<Scalars>}
 */
export async function scalarResult() {
  const result = await invoke("plugin:records|scalar_result");
  return result;
}
/**
 * @param {ReallyFlags} x
 */
export async function flagsArg(x) {
  await invoke("plugin:records|flags_arg", { x: x });
}
/**
 * @returns {Promise<ReallyFlags>}
 */
export async function flagsResult() {
  const result = await invoke("plugin:records|flags_result");
  return result;
}
/**
 * @param {Aggregates} x
 */
export async function aggregateArg(x) {
  await invoke("plugin:records|aggregate_arg", { x: x });
}
/**
 * @returns {Promise<Aggregates>}
 */
export async function aggregateResult() {
  const result = await invoke("plugin:records|aggregate_result");
  return result;
}
/**
 * @param {TupleTypedef2} e
 * @returns {Promise<number>}
 */
export async function typedefInout(e) {
  const result = await invoke("plugin:records|typedef_inout", { e: e });
  return result;
}
