interface Tauri {
  invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
}
declare global {
  interface Window {
    __TAURI__: { tauri: Tauri };
  }
}
const { invoke } = window.__TAURI__.tauri;
export type Error = "success" | "failure";
export async function optionTest(): Promise<string | null> {
  const result = await invoke<string | null>("plugin:imports|option_test");
  return result;
}
