declare global {
	interface Window {
		__TAURI_INVOKE__<T>(
			cmd: string,
			args?: Record<string, unknown>,
		): Promise<T>;
	}
}
const invoke = window.__TAURI_INVOKE__;
if (!window.__TAURI_BINDGEN_VERSION_CHECK__) {
	invoke("plugin|conventions:48646a1b1c089063e7b03a4c1dd9f5ad").catch(() =>
		console.error(
			"The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`.",
		),
	);
}

export interface LudicrousSpeed {
	howFastAreYouGoing: number;
	iAmGoingExtremelySlow: bigint;
}
export async function kebabCase(): Promise<void> {
	await invoke<void>("plugin:conventions|kebab-case");
}
export async function foo(x: LudicrousSpeed): Promise<void> {
	await invoke<void>("plugin:conventions|foo", { x: x });
}
export async function functionWithDashes(): Promise<void> {
	await invoke<void>("plugin:conventions|function-with-dashes");
}
export async function functionWithNoWeirdCharacters(): Promise<void> {
	await invoke<void>("plugin:conventions|function-with-no-weird-characters");
}
export async function apple(): Promise<void> {
	await invoke<void>("plugin:conventions|apple");
}
export async function applePear(): Promise<void> {
	await invoke<void>("plugin:conventions|apple-pear");
}
export async function applePearGrape(): Promise<void> {
	await invoke<void>("plugin:conventions|apple-pear-grape");
}
export async function a0(): Promise<void> {
	await invoke<void>("plugin:conventions|a0");
}
export async function isXml(): Promise<void> {
	await invoke<void>("plugin:conventions|is-XML");
}
export async function explicit(): Promise<void> {
	await invoke<void>("plugin:conventions|explicit");
}
export async function explicitKebab(): Promise<void> {
	await invoke<void>("plugin:conventions|explicit-kebab");
}
export async function bool(): Promise<void> {
	await invoke<void>("plugin:conventions|bool");
}

