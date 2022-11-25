declare global {
	interface Window {
		__TAURI_INVOKE__<T>(
			cmd: string,
			args?: Record<string, unknown>,
		): Promise<T>;
	}
}
const invoke = window.__TAURI_INVOKE__;
export async function float32Param(x: number): Promise<void> {
	await invoke<void>("plugin:b2ded0ef970e6596|float32-param", { x: x });
}
export async function float64Param(x: number): Promise<void> {
	await invoke<void>("plugin:b2ded0ef970e6596|float64-param", { x: x });
}
export async function float32Result(): Promise<number> {
	const result = await invoke<number>("plugin:b2ded0ef970e6596|float32-result");
	return result;
}
export async function float64Result(): Promise<number> {
	const result = await invoke<number>("plugin:b2ded0ef970e6596|float64-result");
	return result;
}

