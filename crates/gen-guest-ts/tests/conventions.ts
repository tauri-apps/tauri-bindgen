interface Tauri {
  invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
}
declare global {
  interface Window {
    __TAURI__: { tauri: Tauri };
  }
}
const { invoke } = window.__TAURI__.tauri;
export interface LudicrousSpeed {
  howFastAreYouGoing: number;
  iAmGoingExtremelySlow: bigint;
}
export async function kebabCase(): Promise<void> {
  await invoke<void>("plugin:imports|kebab_case");
}
export async function foo(x: LudicrousSpeed): Promise<void> {
  await invoke<void>("plugin:imports|foo", { x: x });
}
export async function functionWithDashes(): Promise<void> {
  await invoke<void>("plugin:imports|function_with_dashes");
}
export async function functionWithNoWeirdCharacters(): Promise<void> {
  await invoke<void>("plugin:imports|function_with_no_weird_characters");
}
export async function apple(): Promise<void> {
  await invoke<void>("plugin:imports|apple");
}
export async function applePear(): Promise<void> {
  await invoke<void>("plugin:imports|apple_pear");
}
export async function applePearGrape(): Promise<void> {
  await invoke<void>("plugin:imports|apple_pear_grape");
}
export async function a0(): Promise<void> {
  await invoke<void>("plugin:imports|a0");
}
export async function isXml(): Promise<void> {
  await invoke<void>("plugin:imports|is_xml");
}
export async function explicit(): Promise<void> {
  await invoke<void>("plugin:imports|explicit");
}
export async function explicitKebab(): Promise<void> {
  await invoke<void>("plugin:imports|explicit_kebab");
}
export async function bool(): Promise<void> {
  await invoke<void>("plugin:imports|bool");
}
