declare global {
  interface Window {
    __TAURI_INVOKE__<T>(
      cmd: string,
      args?: Record<string, unknown>
    ): Promise<T>;
  }
}
const invoke = window.__TAURI_INVOKE__;
if (!window.__TAURI_BINDGEN_VERSION_CHECK__) {
  invoke("plugin|records:e6872cf01241a6f3e4c4bedaa609dbeb").catch(() =>
    console.error(
      "The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`."
    )
  );
}

export interface Empty {}
/**
 * A record containing two scalar fields
 * that both have the same type
 */
export interface Scalars {
  /**
   * The first field, named a
   */
  a: number;
  /**
   * The second field, named b
   */
  b: number;
}
/**
 * A record that is really just flags
 * All of the fields are bool
 */
export interface ReallyFlags {
  a: boolean;
  b: boolean;
  c: boolean;
  d: boolean;
  e: boolean;
  f: boolean;
  g: boolean;
  h: boolean;
  i: boolean;
}
export interface Aggregates {
  a: Scalars;
  b: number;
  c: Empty;
  d: string;
  e: ReallyFlags;
}
export type TupleTypedef = [number];
export type IntTypedef = number;
export type TupleTypedef2 = [IntTypedef];
export async function tupleArg(x: [string, number]): Promise<void> {
  await invoke<void>("plugin:records|tuple-arg", { x: x });
}
export async function tupleResult(): Promise<[string, number]> {
  const result = await invoke<[string, number]>("plugin:records|tuple-result");
  return result;
}
export async function emptyArg(x: Empty): Promise<void> {
  await invoke<void>("plugin:records|empty-arg", { x: x });
}
export async function emptyResult(): Promise<Empty> {
  const result = await invoke<Empty>("plugin:records|empty-result");
  return result;
}
export async function scalarArg(x: Scalars): Promise<void> {
  await invoke<void>("plugin:records|scalar-arg", { x: x });
}
export async function scalarResult(): Promise<Scalars> {
  const result = await invoke<Scalars>("plugin:records|scalar-result");
  return result;
}
export async function flagsArg(x: ReallyFlags): Promise<void> {
  await invoke<void>("plugin:records|flags-arg", { x: x });
}
export async function flagsResult(): Promise<ReallyFlags> {
  const result = await invoke<ReallyFlags>("plugin:records|flags-result");
  return result;
}
export async function aggregateArg(x: Aggregates): Promise<void> {
  await invoke<void>("plugin:records|aggregate-arg", { x: x });
}
export async function aggregateResult(): Promise<Aggregates> {
  const result = await invoke<Aggregates>("plugin:records|aggregate-result");
  return result;
}
export async function typedefInout(e: TupleTypedef2): Promise<number> {
  const result = await invoke<number>("plugin:records|typedef-inout", { e: e });
  return result;
}
