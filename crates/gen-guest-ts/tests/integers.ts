declare global {
  interface Window {
    __TAURI_INVOKE__<T>(
      cmd: string,
      args?: Record<string, unknown>
    ): Promise<T>;
  }
}
const invoke = window.__TAURI_INVOKE__;
const idlHash = "279b557e344c2e05";
export async function a1(x: number): Promise<void> {
  await invoke<void>("plugin:integers|a1", { idlHash, x: x });
}
export async function a2(x: number): Promise<void> {
  await invoke<void>("plugin:integers|a2", { idlHash, x: x });
}
export async function a3(x: number): Promise<void> {
  await invoke<void>("plugin:integers|a3", { idlHash, x: x });
}
export async function a4(x: number): Promise<void> {
  await invoke<void>("plugin:integers|a4", { idlHash, x: x });
}
export async function a5(x: number): Promise<void> {
  await invoke<void>("plugin:integers|a5", { idlHash, x: x });
}
export async function a6(x: number): Promise<void> {
  await invoke<void>("plugin:integers|a6", { idlHash, x: x });
}
export async function a7(x: bigint): Promise<void> {
  await invoke<void>("plugin:integers|a7", { idlHash, x: x });
}
export async function a8(x: bigint): Promise<void> {
  await invoke<void>("plugin:integers|a8", { idlHash, x: x });
}
export async function a9(
  p1: number,
  p2: number,
  p3: number,
  p4: number,
  p5: number,
  p6: number,
  p7: bigint,
  p8: bigint
): Promise<void> {
  await invoke<void>("plugin:integers|a9", {
    idlHash,
    p1: p1,
    p2: p2,
    p3: p3,
    p4: p4,
    p5: p5,
    p6: p6,
    p7: p7,
    p8: p8,
  });
}
export async function r1(): Promise<number> {
  const result = await invoke<number>("plugin:integers|r1", { idlHash });
  return result;
}
export async function r2(): Promise<number> {
  const result = await invoke<number>("plugin:integers|r2", { idlHash });
  return result;
}
export async function r3(): Promise<number> {
  const result = await invoke<number>("plugin:integers|r3", { idlHash });
  return result;
}
export async function r4(): Promise<number> {
  const result = await invoke<number>("plugin:integers|r4", { idlHash });
  return result;
}
export async function r5(): Promise<number> {
  const result = await invoke<number>("plugin:integers|r5", { idlHash });
  return result;
}
export async function r6(): Promise<number> {
  const result = await invoke<number>("plugin:integers|r6", { idlHash });
  return result;
}
export async function r7(): Promise<bigint> {
  const result = await invoke<bigint>("plugin:integers|r7", { idlHash });
  return result;
}
export async function r8(): Promise<bigint> {
  const result = await invoke<bigint>("plugin:integers|r8", { idlHash });
  return result;
}
export async function pairRet(): Promise<[bigint, number]> {
  const result = await invoke<[bigint, number]>("plugin:integers|pair-ret", {
    idlHash,
  });
  return result;
}
