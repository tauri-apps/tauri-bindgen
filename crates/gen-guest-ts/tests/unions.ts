interface Tauri {
  invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
}
declare global {
  interface Window {
    __TAURI__: { tauri: Tauri };
  }
}
const { invoke } = window.__TAURI__.tauri;
/**
 * A union of all of the integral types
 */
export type AllIntegers =
  | AllIntegers0
  | AllIntegers1
  | AllIntegers2
  | AllIntegers3
  | AllIntegers4
  | AllIntegers5
  | AllIntegers6
  | AllIntegers7
  | AllIntegers8;
/**
 * Bool is equivalent to a 1 bit integer
 * and is treated that way in some languages
 */
export interface AllIntegers0 {
  tag: 0;
  val: boolean;
}
export interface AllIntegers1 {
  tag: 1;
  val: number;
}
export interface AllIntegers2 {
  tag: 2;
  val: number;
}
export interface AllIntegers3 {
  tag: 3;
  val: number;
}
export interface AllIntegers4 {
  tag: 4;
  val: bigint;
}
export interface AllIntegers5 {
  tag: 5;
  val: number;
}
export interface AllIntegers6 {
  tag: 6;
  val: number;
}
export interface AllIntegers7 {
  tag: 7;
  val: number;
}
export interface AllIntegers8 {
  tag: 8;
  val: bigint;
}
export type AllFloats = AllFloats0 | AllFloats1;
export interface AllFloats0 {
  tag: 0;
  val: number;
}
export interface AllFloats1 {
  tag: 1;
  val: number;
}
export type AllText = AllText0 | AllText1;
export interface AllText0 {
  tag: 0;
  val: string;
}
export interface AllText1 {
  tag: 1;
  val: string;
}
export type DuplicatedS32 = DuplicatedS320 | DuplicatedS321 | DuplicatedS322;
/**
 * The first s32
 */
export interface DuplicatedS320 {
  tag: 0;
  val: number;
}
/**
 * The second s32
 */
export interface DuplicatedS321 {
  tag: 1;
  val: number;
}
/**
 * The third s32
 */
export interface DuplicatedS322 {
  tag: 2;
  val: number;
}
/**
 * A type containing numeric types that are distinct in most languages
 */
export type DistinguishableNum = DistinguishableNum0 | DistinguishableNum1;
/**
 * A Floating Point Number
 */
export interface DistinguishableNum0 {
  tag: 0;
  val: number;
}
/**
 * A Signed Integer
 */
export interface DistinguishableNum1 {
  tag: 1;
  val: bigint;
}
export async function addOneInteger(num: AllIntegers): Promise<AllIntegers> {
  const result = await invoke<AllIntegers>(
    "plugin:import_unions|add_one_integer",
    { num: num }
  );
  return result;
}
export async function addOneFloat(num: AllFloats): Promise<AllFloats> {
  const result = await invoke<AllFloats>("plugin:import_unions|add_one_float", {
    num: num,
  });
  return result;
}
export async function replaceFirstChar(
  text: AllText,
  letter: string
): Promise<AllText> {
  const result = await invoke<AllText>(
    "plugin:import_unions|replace_first_char",
    { text: text, letter: letter }
  );
  return result;
}
export async function identifyInteger(num: AllIntegers): Promise<number> {
  const result = await invoke<number>("plugin:import_unions|identify_integer", {
    num: num,
  });
  return result;
}
export async function identifyFloat(num: AllFloats): Promise<number> {
  const result = await invoke<number>("plugin:import_unions|identify_float", {
    num: num,
  });
  return result;
}
export async function identifyText(text: AllText): Promise<number> {
  const result = await invoke<number>("plugin:import_unions|identify_text", {
    text: text,
  });
  return result;
}
export async function addOneDuplicated(
  num: DuplicatedS32
): Promise<DuplicatedS32> {
  const result = await invoke<DuplicatedS32>(
    "plugin:import_unions|add_one_duplicated",
    { num: num }
  );
  return result;
}
export async function identifyDuplicated(num: DuplicatedS32): Promise<number> {
  const result = await invoke<number>(
    "plugin:import_unions|identify_duplicated",
    { num: num }
  );
  return result;
}
export async function addOneDistinguishableNum(
  num: DistinguishableNum
): Promise<DistinguishableNum> {
  const result = await invoke<DistinguishableNum>(
    "plugin:import_unions|add_one_distinguishable_num",
    { num: num }
  );
  return result;
}
export async function identifyDistinguishableNum(
  num: DistinguishableNum
): Promise<number> {
  const result = await invoke<number>(
    "plugin:import_unions|identify_distinguishable_num",
    { num: num }
  );
  return result;
}
