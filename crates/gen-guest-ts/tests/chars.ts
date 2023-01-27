
declare global {
  interface Window {
    __TAURI_INVOKE__<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
  }
}
const invoke = window.__TAURI_INVOKE__;/**
* A function that accepts a character
*/
export async function takeChar(x: string): Promise<[]> {
  const result = await invoke<[]>("plugin:58d944fc9a2c8431|take-char",{x: x});
  return result
}
/**
* A function that returns a character
*/
export async function returnChar(): Promise<string> {
  const result = await invoke<string>("plugin:58d944fc9a2c8431|return-char",);
  return result
}

