declare global {
  interface Window {
    __TAURI_INVOKE__<T>(
      cmd: string,
      args?: Record<string, unknown>
    ): Promise<T>;
  }
}
const invoke = window.__TAURI_INVOKE__;
const idlHash = "16c3ebd2deefea81";
export async function a(x: string): Promise<void> {
  await invoke<void>("plugin:strings|a", { idlHash, x: x });
}
export async function b(): Promise<string> {
  const result = await invoke<string>("plugin:strings|b", { idlHash });
  return result;
}
export async function c(a: string, b: string): Promise<string> {
  const result = await invoke<string>("plugin:strings|c", {
    idlHash,
    a: a,
    b: b,
  });
  return result;
}
