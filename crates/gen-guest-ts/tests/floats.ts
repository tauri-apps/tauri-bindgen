declare global {
	interface Window {
		__TAURI_INVOKE__<T>(
			cmd: string,
			args?: Record<string, unknown>,
		): Promise<T>;
	}
}
const invoke = window.__TAURI_INVOKE__;
export async function float32Param(x: number): Promise<[]> {
	const result = await invoke<[]>("plugin:979575fda4ffb8b9|float32-param", {
		x: x,
	});
	return result;
}
export async function float64Param(x: number): Promise<[]> {
	const result = await invoke<[]>("plugin:979575fda4ffb8b9|float64-param", {
		x: x,
	});
	return result;
}
export async function float32Result(): Promise<number> {
	const result = await invoke<number>("plugin:979575fda4ffb8b9|float32-result");
	return result;
}
export async function float64Result(): Promise<number> {
	const result = await invoke<number>("plugin:979575fda4ffb8b9|float64-result");
	return result;
}

