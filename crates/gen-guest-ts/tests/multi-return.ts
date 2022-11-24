declare global {
  interface Window {
    __TAURI_INVOKE__<T>(
      cmd: string,
      args?: Record<string, unknown>
    ): Promise<T>;
  }
}
const invoke = window.__TAURI_INVOKE__;
const idlHash = "d238f57052cdcb90";
export async function mra(): Promise<void> {
  await invoke<void>("plugin:multi_return|mra", { idlHash });
}
export async function mrb(): Promise<void> {
  await invoke<void>("plugin:multi_return|mrb", { idlHash });
}
export async function mrc(): Promise<number> {
  const result = await invoke<number>("plugin:multi_return|mrc", { idlHash });
  return result;
}
export async function mrd(): Promise<number> {
  const result = await invoke<number>("plugin:multi_return|mrd", { idlHash });
  return result;
}
export async function mre(): Promise<[number, number]> {
  const result = await invoke<[number, number]>("plugin:multi_return|mre", {
    idlHash,
  });
  return result;
}
