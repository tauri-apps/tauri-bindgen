
declare global {
  interface Window {
    __TAURI_INVOKE__<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
  }
}
const invoke = window.__TAURI_INVOKE__;export async function simpleList1(l: Uint32Array): Promise<[]> {
  const result = await invoke<[]>("plugin:e8600e8d0423cbdb|simple-list1",{l: l});
  return result
}
export async function simpleList2(): Promise<Uint32Array> {
  const result = await invoke<Uint32Array>("plugin:e8600e8d0423cbdb|simple-list2",);
  return result
}
export async function simpleList4(l: Uint32Array[]): Promise<Uint32Array[]> {
  const result = await invoke<Uint32Array[]>("plugin:e8600e8d0423cbdb|simple-list4",{l: l});
  return result
}

