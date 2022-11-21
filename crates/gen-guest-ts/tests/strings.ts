interface Tauri {
  invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
}
declare global {
  interface Window {
    __TAURI__: { tauri: Tauri };
  }
}
const { invoke } = window.__TAURI__.tauri;
export async function a(x: string): Promise<void> {
  await invoke<void>("plugin:imports|a", { x: x });
}
export async function b(): Promise<string> {
  const result = await invoke<string>("plugin:imports|b");
  return result;
}
export async function c(a: string, b: string): Promise<string> {
  const result = await invoke<string>("plugin:imports|c", { a: a, b: b });
  return result;
}
