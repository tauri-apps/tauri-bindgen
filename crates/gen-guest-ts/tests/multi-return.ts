
declare global {
  interface Window {
    __TAURI_INVOKE__<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
  }
}
const invoke = window.__TAURI_INVOKE__;export async function mra(): Promise<[]> {
  const result = await invoke<[]>("plugin:def17a258c1e4f4d|mra",);
  return result
}
export async function mrb(): Promise<void> {
  await invoke<void>("plugin:def17a258c1e4f4d|mrb",);
}
export async function mrc(): Promise<number> {
  const result = await invoke<number>("plugin:def17a258c1e4f4d|mrc",);
  return result
}
export async function mrd(): Promise<number> {
  const result = await invoke<number>("plugin:def17a258c1e4f4d|mrd",);
  return result
}
export async function mre(): Promise<[number, number]> {
  const result = await invoke<[number, number]>("plugin:def17a258c1e4f4d|mre",);
  return result
}

