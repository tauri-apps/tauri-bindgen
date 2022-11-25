declare global {
	interface Window {
		__TAURI_INVOKE__<T>(
			cmd: string,
			args?: Record<string, unknown>,
		): Promise<T>;
	}
}
const invoke = window.__TAURI_INVOKE__;
export interface LudicrousSpeed {
	howFastAreYouGoing: number;
	iAmGoingExtremelySlow: bigint;
}
export async function kebabCase(): Promise<void> {
	await invoke<void>("plugin:48646a1b|kebab-case");
}
export async function foo(x: LudicrousSpeed): Promise<void> {
	await invoke<void>("plugin:48646a1b|foo", { x: x });
}
export async function functionWithDashes(): Promise<void> {
	await invoke<void>("plugin:48646a1b|function-with-dashes");
}
export async function functionWithNoWeirdCharacters(): Promise<void> {
	await invoke<void>("plugin:48646a1b|function-with-no-weird-characters");
}
export async function apple(): Promise<void> {
	await invoke<void>("plugin:48646a1b|apple");
}
export async function applePear(): Promise<void> {
	await invoke<void>("plugin:48646a1b|apple-pear");
}
export async function applePearGrape(): Promise<void> {
	await invoke<void>("plugin:48646a1b|apple-pear-grape");
}
export async function a0(): Promise<void> {
	await invoke<void>("plugin:48646a1b|a0");
}
export async function isXml(): Promise<void> {
	await invoke<void>("plugin:48646a1b|is-XML");
}
export async function explicit(): Promise<void> {
	await invoke<void>("plugin:48646a1b|explicit");
}
export async function explicitKebab(): Promise<void> {
	await invoke<void>("plugin:48646a1b|explicit-kebab");
}
export async function bool(): Promise<void> {
	await invoke<void>("plugin:48646a1b|bool");
}

