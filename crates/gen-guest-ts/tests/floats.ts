interface Tauri {
  invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
}
declare global {
  interface Window {
    __TAURI__: { tauri: Tauri };
  }
}
const { invoke } = window.__TAURI__.tauri;
export async function float32Param(x: number): Promise<void> {
  await invoke<void>("plugin:imports|float32_param", { x: x });
}
export async function float64Param(x: number): Promise<void> {
  await invoke<void>("plugin:imports|float64_param", { x: x });
}
export async function float32Result(): Promise<number> {
  const result = await invoke<number>("plugin:imports|float32_result");
  return result;
}
export async function float64Result(): Promise<number> {
  const result = await invoke<number>("plugin:imports|float64_result");
  return result;
}
