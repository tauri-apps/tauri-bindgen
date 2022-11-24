declare global {
  interface Window {
    __TAURI_INVOKE__<T>(
      cmd: string,
      args?: Record<string, unknown>
    ): Promise<T>;
  }
}
const invoke = window.__TAURI_INVOKE__;
const idlHash = "ebb2d6f0441e00a0";
export async function f1(): Promise<void> {
  await invoke<void>("plugin:simple|f1", { idlHash });
}
export async function f2(a: number): Promise<void> {
  await invoke<void>("plugin:simple|f2", { idlHash, a: a });
}
export async function f3(a: number, b: number): Promise<void> {
  await invoke<void>("plugin:simple|f3", { idlHash, a: a, b: b });
}
export async function f4(): Promise<number> {
  const result = await invoke<number>("plugin:simple|f4", { idlHash });
  return result;
}
export async function f5(): Promise<[number, number]> {
  const result = await invoke<[number, number]>("plugin:simple|f5", {
    idlHash,
  });
  return result;
}
export async function f6(
  a: number,
  b: number,
  c: number
): Promise<[number, number, number]> {
  const result = await invoke<[number, number, number]>("plugin:simple|f6", {
    idlHash,
    a: a,
    b: b,
    c: c,
  });
  return result;
}
