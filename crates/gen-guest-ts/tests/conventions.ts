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
export async function kebabCase(): Promise<[]> {
	const result = await invoke<[]>("plugin:3d46778dded1f0fb|kebab-case");
	return result;
}
export async function foo(x: LudicrousSpeed): Promise<[]> {
	const result = await invoke<[]>("plugin:3d46778dded1f0fb|foo", { x: x });
	return result;
}
export async function functionWithDashes(): Promise<[]> {
	const result = await invoke<[]>(
		"plugin:3d46778dded1f0fb|function-with-dashes",
	);
	return result;
}
export async function functionWithNoWeirdCharacters(): Promise<[]> {
	const result = await invoke<[]>(
		"plugin:3d46778dded1f0fb|function-with-no-weird-characters",
	);
	return result;
}
export async function apple(): Promise<[]> {
	const result = await invoke<[]>("plugin:3d46778dded1f0fb|apple");
	return result;
}
export async function applePear(): Promise<[]> {
	const result = await invoke<[]>("plugin:3d46778dded1f0fb|apple-pear");
	return result;
}
export async function applePearGrape(): Promise<[]> {
	const result = await invoke<[]>("plugin:3d46778dded1f0fb|apple-pear-grape");
	return result;
}
export async function a0(): Promise<[]> {
	const result = await invoke<[]>("plugin:3d46778dded1f0fb|a0");
	return result;
}
export async function isXml(): Promise<[]> {
	const result = await invoke<[]>("plugin:3d46778dded1f0fb|is-XML");
	return result;
}
export async function explicit(): Promise<[]> {
	const result = await invoke<[]>("plugin:3d46778dded1f0fb|explicit");
	return result;
}
export async function explicitKebab(): Promise<[]> {
	const result = await invoke<[]>("plugin:3d46778dded1f0fb|explicit-kebab");
	return result;
}
export async function bool(): Promise<[]> {
	const result = await invoke<[]>("plugin:3d46778dded1f0fb|bool");
	return result;
}

