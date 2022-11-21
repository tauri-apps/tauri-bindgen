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
 * A function that accepts a character
 */
export async function takeChar(x: string): Promise<void> {
  await invoke<void>("plugin:imports|take_char", { x: x });
}
/**
 * A function that returns a character
 */
export async function returnChar(): Promise<string> {
  const result = await invoke<string>("plugin:imports|return_char");
  return result;
}
