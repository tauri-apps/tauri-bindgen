interface Tauri {
  invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
}
declare global {
  interface Window {
    __TAURI__: { tauri: Tauri };
  }
}
const { invoke } = window.__TAURI__.tauri;
export async function f1(): Promise<void> {
  await invoke<void>("plugin:imports|f1");
}
export async function f2(a: number): Promise<void> {
  await invoke<void>("plugin:imports|f2", { a: a });
}
export async function f3(a: number, b: number): Promise<void> {
  await invoke<void>("plugin:imports|f3", { a: a, b: b });
}
export async function f4(): Promise<number> {
  const result = await invoke<number>("plugin:imports|f4");
  return result;
}
export async function f5(): Promise<[number, number]> {
  const result = await invoke<[number, number]>("plugin:imports|f5");
  return result;
}
export async function f6(
  a: number,
  b: number,
  c: number
): Promise<[number, number, number]> {
  const result = await invoke<[number, number, number]>("plugin:imports|f6", {
    a: a,
    b: b,
    c: c,
  });
  return result;
}
