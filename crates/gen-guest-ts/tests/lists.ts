interface Tauri {
  invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
}
declare global {
  interface Window {
    __TAURI__: { tauri: Tauri };
  }
}
const { invoke } = window.__TAURI__.tauri;
export interface SomeRecord {
  x: string;
  y: OtherRecord;
  z: OtherRecord[];
  c1: number;
  c2: bigint;
  c3: number;
  c4: bigint;
}
export interface OtherRecord {
  a1: number;
  a2: bigint;
  a3: number;
  a4: bigint;
  b: string;
  c: Uint8Array;
}
export type SomeVariant =
  | SomeVariantA
  | SomeVariantB
  | SomeVariantC
  | SomeVariantD;
export interface SomeVariantA {
  tag: "a";
  val: string;
}
export interface SomeVariantB {
  tag: "b";
}
export interface SomeVariantC {
  tag: "c";
  val: number;
}
export interface SomeVariantD {
  tag: "d";
  val: OtherVariant[];
}
export type OtherVariant = OtherVariantA | OtherVariantB | OtherVariantC;
export interface OtherVariantA {
  tag: "a";
}
export interface OtherVariantB {
  tag: "b";
  val: number;
}
export interface OtherVariantC {
  tag: "c";
  val: string;
}
export type LoadStoreAllSizes = [
  string,
  number,
  number,
  number,
  number,
  number,
  number,
  bigint,
  bigint,
  number,
  number,
  string
][];
export async function listU8Param(x: Uint8Array): Promise<void> {
  await invoke<void>("plugin:import_lists|list_u8_param", { x: x });
}
export async function listU16Param(x: Uint16Array): Promise<void> {
  await invoke<void>("plugin:import_lists|list_u16_param", { x: x });
}
export async function listU32Param(x: Uint32Array): Promise<void> {
  await invoke<void>("plugin:import_lists|list_u32_param", { x: x });
}
export async function listU64Param(x: BigUint64Array): Promise<void> {
  await invoke<void>("plugin:import_lists|list_u64_param", { x: x });
}
export async function listS8Param(x: Int8Array): Promise<void> {
  await invoke<void>("plugin:import_lists|list_s8_param", { x: x });
}
export async function listS16Param(x: Int16Array): Promise<void> {
  await invoke<void>("plugin:import_lists|list_s16_param", { x: x });
}
export async function listS32Param(x: Int32Array): Promise<void> {
  await invoke<void>("plugin:import_lists|list_s32_param", { x: x });
}
export async function listS64Param(x: BigInt64Array): Promise<void> {
  await invoke<void>("plugin:import_lists|list_s64_param", { x: x });
}
export async function listFloat32Param(x: Float32Array): Promise<void> {
  await invoke<void>("plugin:import_lists|list_float32_param", { x: x });
}
export async function listFloat64Param(x: Float64Array): Promise<void> {
  await invoke<void>("plugin:import_lists|list_float64_param", { x: x });
}
export async function listU8Ret(): Promise<Uint8Array> {
  const result = await invoke<Uint8Array>("plugin:import_lists|list_u8_ret");
  return result;
}
export async function listU16Ret(): Promise<Uint16Array> {
  const result = await invoke<Uint16Array>("plugin:import_lists|list_u16_ret");
  return result;
}
export async function listU32Ret(): Promise<Uint32Array> {
  const result = await invoke<Uint32Array>("plugin:import_lists|list_u32_ret");
  return result;
}
export async function listU64Ret(): Promise<BigUint64Array> {
  const result = await invoke<BigUint64Array>(
    "plugin:import_lists|list_u64_ret"
  );
  return result;
}
export async function listS8Ret(): Promise<Int8Array> {
  const result = await invoke<Int8Array>("plugin:import_lists|list_s8_ret");
  return result;
}
export async function listS16Ret(): Promise<Int16Array> {
  const result = await invoke<Int16Array>("plugin:import_lists|list_s16_ret");
  return result;
}
export async function listS32Ret(): Promise<Int32Array> {
  const result = await invoke<Int32Array>("plugin:import_lists|list_s32_ret");
  return result;
}
export async function listS64Ret(): Promise<BigInt64Array> {
  const result = await invoke<BigInt64Array>(
    "plugin:import_lists|list_s64_ret"
  );
  return result;
}
export async function listFloat32Ret(): Promise<Float32Array> {
  const result = await invoke<Float32Array>(
    "plugin:import_lists|list_float32_ret"
  );
  return result;
}
export async function listFloat64Ret(): Promise<Float64Array> {
  const result = await invoke<Float64Array>(
    "plugin:import_lists|list_float64_ret"
  );
  return result;
}
export async function tupleList(
  x: [number, number][]
): Promise<[bigint, number][]> {
  const result = await invoke<[bigint, number][]>(
    "plugin:import_lists|tuple_list",
    { x: x }
  );
  return result;
}
export async function stringListArg(a: string[]): Promise<void> {
  await invoke<void>("plugin:import_lists|string_list_arg", { a: a });
}
export async function stringListRet(): Promise<string[]> {
  const result = await invoke<string[]>("plugin:import_lists|string_list_ret");
  return result;
}
export async function tupleStringList(
  x: [number, string][]
): Promise<[string, number][]> {
  const result = await invoke<[string, number][]>(
    "plugin:import_lists|tuple_string_list",
    { x: x }
  );
  return result;
}
export async function stringList(x: string[]): Promise<string[]> {
  const result = await invoke<string[]>("plugin:import_lists|string_list", {
    x: x,
  });
  return result;
}
export async function recordList(x: SomeRecord[]): Promise<OtherRecord[]> {
  const result = await invoke<OtherRecord[]>(
    "plugin:import_lists|record_list",
    { x: x }
  );
  return result;
}
export async function recordListReverse(
  x: OtherRecord[]
): Promise<SomeRecord[]> {
  const result = await invoke<SomeRecord[]>(
    "plugin:import_lists|record_list_reverse",
    { x: x }
  );
  return result;
}
export async function variantList(x: SomeVariant[]): Promise<OtherVariant[]> {
  const result = await invoke<OtherVariant[]>(
    "plugin:import_lists|variant_list",
    { x: x }
  );
  return result;
}
export async function loadStoreEverything(
  a: LoadStoreAllSizes
): Promise<LoadStoreAllSizes> {
  const result = await invoke<LoadStoreAllSizes>(
    "plugin:import_lists|load_store_everything",
    { a: a }
  );
  return result;
}
