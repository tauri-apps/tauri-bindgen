interface Tauri {
  invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
}
declare global {
  interface Window {
    __TAURI__: { tauri: Tauri };
  }
}
const { invoke } = window.__TAURI__.tauri;
export async function simpleList1(l: Uint32Array): Promise<void> {
  await invoke<void>("plugin:imports|simple_list1", { l: l });
}
export async function simpleList2(): Promise<Uint32Array> {
  const result = await invoke<Uint32Array>("plugin:imports|simple_list2");
  return result;
}
export async function simpleList4(l: Uint32Array[]): Promise<Uint32Array[]> {
  const result = await invoke<Uint32Array[]>("plugin:imports|simple_list4", {
    l: l,
  });
  return result;
}
