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
  invoke("plugin|integers:279b557e344c2e05853f5c89d6d511dc").catch(() =>
    console.error(
      "The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`."
    )
  );
}

export async function a1(x: number): Promise<void> {
  await invoke<void>("plugin:integers|a1", { x: x });
}
export async function a2(x: number): Promise<void> {
  await invoke<void>("plugin:integers|a2", { x: x });
}
export async function a3(x: number): Promise<void> {
  await invoke<void>("plugin:integers|a3", { x: x });
}
export async function a4(x: number): Promise<void> {
  await invoke<void>("plugin:integers|a4", { x: x });
}
export async function a5(x: number): Promise<void> {
  await invoke<void>("plugin:integers|a5", { x: x });
}
export async function a6(x: number): Promise<void> {
  await invoke<void>("plugin:integers|a6", { x: x });
}
export async function a7(x: bigint): Promise<void> {
  await invoke<void>("plugin:integers|a7", { x: x });
}
export async function a8(x: bigint): Promise<void> {
  await invoke<void>("plugin:integers|a8", { x: x });
}
export async function a9(
  p1: number,
  p2: number,
  p3: number,
  p4: number,
  p5: number,
  p6: number,
  p7: bigint,
  p8: bigint
): Promise<void> {
  await invoke<void>("plugin:integers|a9", {
    p1: p1,
    p2: p2,
    p3: p3,
    p4: p4,
    p5: p5,
    p6: p6,
    p7: p7,
    p8: p8,
  });
}
export async function r1(): Promise<number> {
  const result = await invoke<number>("plugin:integers|r1");
  return result;
}
export async function r2(): Promise<number> {
  const result = await invoke<number>("plugin:integers|r2");
  return result;
}
export async function r3(): Promise<number> {
  const result = await invoke<number>("plugin:integers|r3");
  return result;
}
export async function r4(): Promise<number> {
  const result = await invoke<number>("plugin:integers|r4");
  return result;
}
export async function r5(): Promise<number> {
  const result = await invoke<number>("plugin:integers|r5");
  return result;
}
export async function r6(): Promise<number> {
  const result = await invoke<number>("plugin:integers|r6");
  return result;
}
export async function r7(): Promise<bigint> {
  const result = await invoke<bigint>("plugin:integers|r7");
  return result;
}
export async function r8(): Promise<bigint> {
  const result = await invoke<bigint>("plugin:integers|r8");
  return result;
}
export async function pairRet(): Promise<[bigint, number]> {
  const result = await invoke<[bigint, number]>("plugin:integers|pair-ret");
  return result;
}
