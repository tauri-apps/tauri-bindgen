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
  invoke("plugin|simple:ebb2d6f0441e00a02915e2faf10bbe90").catch(() =>
    console.error(
      "The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`."
    )
  );
}

export async function f1(): Promise<void> {
  await invoke<void>("plugin:simple|f1");
}
export async function f2(a: number): Promise<void> {
  await invoke<void>("plugin:simple|f2", { a: a });
}
export async function f3(a: number, b: number): Promise<void> {
  await invoke<void>("plugin:simple|f3", { a: a, b: b });
}
export async function f4(): Promise<number> {
  const result = await invoke<number>("plugin:simple|f4");
  return result;
}
export async function f5(): Promise<[number, number]> {
  const result = await invoke<[number, number]>("plugin:simple|f5");
  return result;
}
export async function f6(
  a: number,
  b: number,
  c: number
): Promise<[number, number, number]> {
  const result = await invoke<[number, number, number]>("plugin:simple|f6", {
    a: a,
    b: b,
    c: c,
  });
  return result;
}
