declare global {
  interface Window {
    __TAURI_INVOKE__<T>(
      cmd: string,
      args?: Record<string, unknown>
    ): Promise<T>;
  }
}
const invoke = window.__TAURI_INVOKE__;
const idlHash = "b2ded0ef970e6596";
export async function float32Param(x: number): Promise<void> {
  await invoke<void>("plugin:floats|float32-param", { idlHash, x: x });
}
export async function float64Param(x: number): Promise<void> {
  await invoke<void>("plugin:floats|float64-param", { idlHash, x: x });
}
export async function float32Result(): Promise<number> {
  const result = await invoke<number>("plugin:floats|float32-result", {
    idlHash,
  });
  return result;
}
export async function float64Result(): Promise<number> {
  const result = await invoke<number>("plugin:floats|float64-result", {
    idlHash,
  });
  return result;
}
