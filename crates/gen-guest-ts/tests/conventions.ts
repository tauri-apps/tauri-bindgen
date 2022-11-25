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
	await invoke<void>("plugin:48646a1b1c089063|kebab-case");
}
export async function foo(x: LudicrousSpeed): Promise<void> {
	await invoke<void>("plugin:48646a1b1c089063|foo", { x: x });
}
export async function functionWithDashes(): Promise<void> {
	await invoke<void>("plugin:48646a1b1c089063|function-with-dashes");
}
export async function functionWithNoWeirdCharacters(): Promise<void> {
	await invoke<void>(
		"plugin:48646a1b1c089063|function-with-no-weird-characters",
	);
}
export async function apple(): Promise<void> {
	await invoke<void>("plugin:48646a1b1c089063|apple");
}
export async function applePear(): Promise<void> {
	await invoke<void>("plugin:48646a1b1c089063|apple-pear");
}
export async function applePearGrape(): Promise<void> {
	await invoke<void>("plugin:48646a1b1c089063|apple-pear-grape");
}
export async function a0(): Promise<void> {
	await invoke<void>("plugin:48646a1b1c089063|a0");
}
export async function isXml(): Promise<void> {
	await invoke<void>("plugin:48646a1b1c089063|is-XML");
}
export async function explicit(): Promise<void> {
	await invoke<void>("plugin:48646a1b1c089063|explicit");
}
export async function explicitKebab(): Promise<void> {
	await invoke<void>("plugin:48646a1b1c089063|explicit-kebab");
}
export async function bool(): Promise<void> {
	await invoke<void>("plugin:48646a1b1c089063|bool");
}

