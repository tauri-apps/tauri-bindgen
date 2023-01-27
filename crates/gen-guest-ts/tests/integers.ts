
declare global {
  interface Window {
    __TAURI_INVOKE__<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
  }
}
const invoke = window.__TAURI_INVOKE__;export async function a1(x: number): Promise<[]> {
  const result = await invoke<[]>("plugin:0c0ef23cf9639264|a1",{x: x});
  return result
}
export async function a2(x: number): Promise<[]> {
  const result = await invoke<[]>("plugin:0c0ef23cf9639264|a2",{x: x});
  return result
}
export async function a3(x: number): Promise<[]> {
  const result = await invoke<[]>("plugin:0c0ef23cf9639264|a3",{x: x});
  return result
}
export async function a4(x: number): Promise<[]> {
  const result = await invoke<[]>("plugin:0c0ef23cf9639264|a4",{x: x});
  return result
}
export async function a5(x: number): Promise<[]> {
  const result = await invoke<[]>("plugin:0c0ef23cf9639264|a5",{x: x});
  return result
}
export async function a6(x: number): Promise<[]> {
  const result = await invoke<[]>("plugin:0c0ef23cf9639264|a6",{x: x});
  return result
}
export async function a7(x: bigint): Promise<[]> {
  const result = await invoke<[]>("plugin:0c0ef23cf9639264|a7",{x: x});
  return result
}
export async function a8(x: bigint): Promise<[]> {
  const result = await invoke<[]>("plugin:0c0ef23cf9639264|a8",{x: x});
  return result
}
export async function a9(p1: number, p2: number, p3: number, p4: number, p5: number, p6: number, p7: bigint, p8: bigint): Promise<[]> {
  const result = await invoke<[]>("plugin:0c0ef23cf9639264|a9",{p1: p1, p2: p2, p3: p3, p4: p4, p5: p5, p6: p6, p7: p7, p8: p8});
  return result
}
export async function r1(): Promise<number> {
  const result = await invoke<number>("plugin:0c0ef23cf9639264|r1",);
  return result
}
export async function r2(): Promise<number> {
  const result = await invoke<number>("plugin:0c0ef23cf9639264|r2",);
  return result
}
export async function r3(): Promise<number> {
  const result = await invoke<number>("plugin:0c0ef23cf9639264|r3",);
  return result
}
export async function r4(): Promise<number> {
  const result = await invoke<number>("plugin:0c0ef23cf9639264|r4",);
  return result
}
export async function r5(): Promise<number> {
  const result = await invoke<number>("plugin:0c0ef23cf9639264|r5",);
  return result
}
export async function r6(): Promise<number> {
  const result = await invoke<number>("plugin:0c0ef23cf9639264|r6",);
  return result
}
export async function r7(): Promise<bigint> {
  const result = await invoke<bigint>("plugin:0c0ef23cf9639264|r7",);
  return result
}
export async function r8(): Promise<bigint> {
  const result = await invoke<bigint>("plugin:0c0ef23cf9639264|r8",);
  return result
}
export async function pairRet(): Promise<[bigint, number]> {
  const result = await invoke<[bigint, number]>("plugin:0c0ef23cf9639264|pair-ret",);
  return result
}

