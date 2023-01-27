
declare global {
  interface Window {
    __TAURI_INVOKE__<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
  }
}
const invoke = window.__TAURI_INVOKE__;export interface SomeRecord {
  x: string,
  y: OtherRecord,
  z: OtherRecord[],
  c1: number,
  c2: bigint,
  c3: number,
  c4: bigint,
};
export interface OtherRecord {
  a1: number,
  a2: bigint,
  a3: number,
  a4: bigint,
  b: string,
  c: Uint8Array,
};
export type SomeVariant = SomeVariantA| SomeVariantB| SomeVariantC| SomeVariantD;
export interface SomeVariantA {
  tag: 'a',
  val: string,
}
export interface SomeVariantB {
  tag: 'b',
}
export interface SomeVariantC {
  tag: 'c',
  val: number,
}
export interface SomeVariantD {
  tag: 'd',
  val: OtherVariant[],
}
export type OtherVariant = OtherVariantA| OtherVariantB| OtherVariantC;
export interface OtherVariantA {
  tag: 'a',
}
export interface OtherVariantB {
  tag: 'b',
  val: number,
}
export interface OtherVariantC {
  tag: 'c',
  val: string,
}
export type LoadStoreAllSizes = [string, number, number, number, number, number, number, bigint, bigint, number, number, string][];
export async function listU8Param(x: Uint8Array): Promise<[]> {
  const result = await invoke<[]>("plugin:3d9d99368dfa9a39|list-u8-param",{x: x});
  return result
}
export async function listU16Param(x: Uint16Array): Promise<[]> {
  const result = await invoke<[]>("plugin:3d9d99368dfa9a39|list-u16-param",{x: x});
  return result
}
export async function listU32Param(x: Uint32Array): Promise<[]> {
  const result = await invoke<[]>("plugin:3d9d99368dfa9a39|list-u32-param",{x: x});
  return result
}
export async function listU64Param(x: BigUint64Array): Promise<[]> {
  const result = await invoke<[]>("plugin:3d9d99368dfa9a39|list-u64-param",{x: x});
  return result
}
export async function listS8Param(x: Int8Array): Promise<[]> {
  const result = await invoke<[]>("plugin:3d9d99368dfa9a39|list-s8-param",{x: x});
  return result
}
export async function listS16Param(x: Int16Array): Promise<[]> {
  const result = await invoke<[]>("plugin:3d9d99368dfa9a39|list-s16-param",{x: x});
  return result
}
export async function listS32Param(x: Int32Array): Promise<[]> {
  const result = await invoke<[]>("plugin:3d9d99368dfa9a39|list-s32-param",{x: x});
  return result
}
export async function listS64Param(x: BigInt64Array): Promise<[]> {
  const result = await invoke<[]>("plugin:3d9d99368dfa9a39|list-s64-param",{x: x});
  return result
}
export async function listFloat32Param(x: Float32Array): Promise<[]> {
  const result = await invoke<[]>("plugin:3d9d99368dfa9a39|list-float32-param",{x: x});
  return result
}
export async function listFloat64Param(x: Float64Array): Promise<[]> {
  const result = await invoke<[]>("plugin:3d9d99368dfa9a39|list-float64-param",{x: x});
  return result
}
export async function listU8Ret(): Promise<Uint8Array> {
  const result = await invoke<Uint8Array>("plugin:3d9d99368dfa9a39|list-u8-ret",);
  return result
}
export async function listU16Ret(): Promise<Uint16Array> {
  const result = await invoke<Uint16Array>("plugin:3d9d99368dfa9a39|list-u16-ret",);
  return result
}
export async function listU32Ret(): Promise<Uint32Array> {
  const result = await invoke<Uint32Array>("plugin:3d9d99368dfa9a39|list-u32-ret",);
  return result
}
export async function listU64Ret(): Promise<BigUint64Array> {
  const result = await invoke<BigUint64Array>("plugin:3d9d99368dfa9a39|list-u64-ret",);
  return result
}
export async function listS8Ret(): Promise<Int8Array> {
  const result = await invoke<Int8Array>("plugin:3d9d99368dfa9a39|list-s8-ret",);
  return result
}
export async function listS16Ret(): Promise<Int16Array> {
  const result = await invoke<Int16Array>("plugin:3d9d99368dfa9a39|list-s16-ret",);
  return result
}
export async function listS32Ret(): Promise<Int32Array> {
  const result = await invoke<Int32Array>("plugin:3d9d99368dfa9a39|list-s32-ret",);
  return result
}
export async function listS64Ret(): Promise<BigInt64Array> {
  const result = await invoke<BigInt64Array>("plugin:3d9d99368dfa9a39|list-s64-ret",);
  return result
}
export async function listFloat32Ret(): Promise<Float32Array> {
  const result = await invoke<Float32Array>("plugin:3d9d99368dfa9a39|list-float32-ret",);
  return result
}
export async function listFloat64Ret(): Promise<Float64Array> {
  const result = await invoke<Float64Array>("plugin:3d9d99368dfa9a39|list-float64-ret",);
  return result
}
export async function tupleList(x: [number, number][]): Promise<[bigint, number][]> {
  const result = await invoke<[bigint, number][]>("plugin:3d9d99368dfa9a39|tuple-list",{x: x});
  return result
}
export async function stringListArg(a: string[]): Promise<[]> {
  const result = await invoke<[]>("plugin:3d9d99368dfa9a39|string-list-arg",{a: a});
  return result
}
export async function stringListRet(): Promise<string[]> {
  const result = await invoke<string[]>("plugin:3d9d99368dfa9a39|string-list-ret",);
  return result
}
export async function tupleStringList(x: [number, string][]): Promise<[string, number][]> {
  const result = await invoke<[string, number][]>("plugin:3d9d99368dfa9a39|tuple-string-list",{x: x});
  return result
}
export async function stringList(x: string[]): Promise<string[]> {
  const result = await invoke<string[]>("plugin:3d9d99368dfa9a39|string-list",{x: x});
  return result
}
export async function recordList(x: SomeRecord[]): Promise<OtherRecord[]> {
  const result = await invoke<OtherRecord[]>("plugin:3d9d99368dfa9a39|record-list",{x: x});
  return result
}
export async function recordListReverse(x: OtherRecord[]): Promise<SomeRecord[]> {
  const result = await invoke<SomeRecord[]>("plugin:3d9d99368dfa9a39|record-list-reverse",{x: x});
  return result
}
export async function variantList(x: SomeVariant[]): Promise<OtherVariant[]> {
  const result = await invoke<OtherVariant[]>("plugin:3d9d99368dfa9a39|variant-list",{x: x});
  return result
}
export async function loadStoreEverything(a: LoadStoreAllSizes): Promise<LoadStoreAllSizes> {
  const result = await invoke<LoadStoreAllSizes>("plugin:3d9d99368dfa9a39|load-store-everything",{a: a});
  return result
}

