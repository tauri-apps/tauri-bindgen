declare global {
  interface Window {
    __TAURI_INVOKE__<T>(
      cmd: string,
      args?: Record<string, unknown>
    ): Promise<T>;
  }
}
const invoke = window.__TAURI_INVOKE__;
if (!window.__TAURI_BINDGEN_VERSION_CHECK__) {
  invoke("plugin|multi_return:d238f57052cdcb9073d14f7a8059345b").catch(() =>
    console.error(
      "The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`."
    )
  );
}

export async function mra(): Promise<void> {
  await invoke<void>("plugin:multi_return|mra");
}
export async function mrb(): Promise<void> {
  await invoke<void>("plugin:multi_return|mrb");
}
export async function mrc(): Promise<number> {
  const result = await invoke<number>("plugin:multi_return|mrc");
  return result;
}
export async function mrd(): Promise<number> {
  const result = await invoke<number>("plugin:multi_return|mrd");
  return result;
}
export async function mre(): Promise<[number, number]> {
  const result = await invoke<[number, number]>("plugin:multi_return|mre");
  return result;
}
