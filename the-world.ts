
interface Tauri {
  invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T>
}

declare global {
  interface Window {
    __TAURI__: { tauri: Tauri };
  }
}
/**
* A function that accepts a character
*/
export async function takeChar(x: string): Promise<void> {
  await window.__TAURI__.tauri.invoke<void>("plugin:imports|take_char",{x: x});
}
/**
* A function that returns a character
*/
export async function returnChar(): Promise<string> {
  const result = await window.__TAURI__.tauri.invoke<string>("plugin:imports|return_char",);
  return result
}

