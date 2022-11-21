interface Tauri {
  invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
}
declare global {
  interface Window {
    __TAURI__: { tauri: Tauri };
  }
}
const { invoke } = window.__TAURI__.tauri;
export async function mra(): Promise<void> {
  await invoke<void>("plugin:imports|mra");
}
export async function mrb(): Promise<void> {
  await invoke<void>("plugin:imports|mrb");
}
export async function mrc(): Promise<number> {
  const result = await invoke<number>("plugin:imports|mrc");
  return result;
}
export async function mrd(): Promise<number> {
  const result = await invoke<number>("plugin:imports|mrd");
  return result;
}
export async function mre(): Promise<[number, number]> {
  const result = await invoke<[number, number]>("plugin:imports|mre");
  return result;
}
