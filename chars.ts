declare global {
  interface Window {
    __TAURI_INVOKE__<T>(
      cmd: string,
      args?: Record<string, unknown>
    ): Promise<T>;
  }
}
const invoke = window.__TAURI_INVOKE__;
/**
 * A function that accepts a character
 */
export async function takeChar(x: string): Promise<void> {
  await invoke<void>("plugin:chars|0", { 0: x });
}
/**
 * A function that returns a character
 */
export async function returnChar(): Promise<string> {
  const result = await invoke<string>("plugin:chars|1");
  return result;
}
