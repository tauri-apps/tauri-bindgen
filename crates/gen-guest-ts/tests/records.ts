declare global {
  interface Window {
    __TAURI_INVOKE__<T>(
      cmd: string,
      args?: Record<string, unknown>
    ): Promise<T>;
  }
}
const invoke = window.__TAURI_INVOKE__;
const idlHash = "e6872cf01241a6f3";
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
  await invoke<void>("plugin:records|tuple-arg", { idlHash, x: x });
}
export async function tupleResult(): Promise<[string, number]> {
  const result = await invoke<[string, number]>("plugin:records|tuple-result", {
    idlHash,
  });
  return result;
}
export async function emptyArg(x: Empty): Promise<void> {
  await invoke<void>("plugin:records|empty-arg", { idlHash, x: x });
}
export async function emptyResult(): Promise<Empty> {
  const result = await invoke<Empty>("plugin:records|empty-result", {
    idlHash,
  });
  return result;
}
export async function scalarArg(x: Scalars): Promise<void> {
  await invoke<void>("plugin:records|scalar-arg", { idlHash, x: x });
}
export async function scalarResult(): Promise<Scalars> {
  const result = await invoke<Scalars>("plugin:records|scalar-result", {
    idlHash,
  });
  return result;
}
export async function flagsArg(x: ReallyFlags): Promise<void> {
  await invoke<void>("plugin:records|flags-arg", { idlHash, x: x });
}
export async function flagsResult(): Promise<ReallyFlags> {
  const result = await invoke<ReallyFlags>("plugin:records|flags-result", {
    idlHash,
  });
  return result;
}
export async function aggregateArg(x: Aggregates): Promise<void> {
  await invoke<void>("plugin:records|aggregate-arg", { idlHash, x: x });
}
export async function aggregateResult(): Promise<Aggregates> {
  const result = await invoke<Aggregates>("plugin:records|aggregate-result", {
    idlHash,
  });
  return result;
}
export async function typedefInout(e: TupleTypedef2): Promise<number> {
  const result = await invoke<number>("plugin:records|typedef-inout", {
    idlHash,
    e: e,
  });
  return result;
}
