declare global {
  interface Window {
    __TAURI_INVOKE__<T>(
      cmd: string,
      args?: Record<string, unknown>
    ): Promise<T>;
  }
}
const invoke = window.__TAURI_INVOKE__;
const idlHash = "d40a3203ef48115d";
export async function simpleList1(l: Uint32Array): Promise<void> {
  await invoke<void>("plugin:simple_lists|simple-list1", { idlHash, l: l });
}
export async function simpleList2(): Promise<Uint32Array> {
  const result = await invoke<Uint32Array>("plugin:simple_lists|simple-list2", {
    idlHash,
  });
  return result;
}
export async function simpleList4(l: Uint32Array[]): Promise<Uint32Array[]> {
  const result = await invoke<Uint32Array[]>(
    "plugin:simple_lists|simple-list4",
    { idlHash, l: l }
  );
  return result;
}
