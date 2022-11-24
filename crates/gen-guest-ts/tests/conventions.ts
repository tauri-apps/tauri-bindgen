declare global {
  interface Window {
    __TAURI_INVOKE__<T>(
      cmd: string,
      args?: Record<string, unknown>
    ): Promise<T>;
  }
}
const invoke = window.__TAURI_INVOKE__;
const idlHash = "48646a1b1c089063";
export interface LudicrousSpeed {
  howFastAreYouGoing: number;
  iAmGoingExtremelySlow: bigint;
}
export async function kebabCase(): Promise<void> {
  await invoke<void>("plugin:conventions|kebab-case", { idlHash });
}
export async function foo(x: LudicrousSpeed): Promise<void> {
  await invoke<void>("plugin:conventions|foo", { idlHash, x: x });
}
export async function functionWithDashes(): Promise<void> {
  await invoke<void>("plugin:conventions|function-with-dashes", { idlHash });
}
export async function functionWithNoWeirdCharacters(): Promise<void> {
  await invoke<void>("plugin:conventions|function-with-no-weird-characters", {
    idlHash,
  });
}
export async function apple(): Promise<void> {
  await invoke<void>("plugin:conventions|apple", { idlHash });
}
export async function applePear(): Promise<void> {
  await invoke<void>("plugin:conventions|apple-pear", { idlHash });
}
export async function applePearGrape(): Promise<void> {
  await invoke<void>("plugin:conventions|apple-pear-grape", { idlHash });
}
export async function a0(): Promise<void> {
  await invoke<void>("plugin:conventions|a0", { idlHash });
}
export async function isXml(): Promise<void> {
  await invoke<void>("plugin:conventions|is-XML", { idlHash });
}
export async function explicit(): Promise<void> {
  await invoke<void>("plugin:conventions|explicit", { idlHash });
}
export async function explicitKebab(): Promise<void> {
  await invoke<void>("plugin:conventions|explicit-kebab", { idlHash });
}
export async function bool(): Promise<void> {
  await invoke<void>("plugin:conventions|bool", { idlHash });
}
