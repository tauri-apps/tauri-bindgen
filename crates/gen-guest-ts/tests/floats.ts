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
	invoke("plugin|floats:b2ded0ef970e65969239249842d626ce").catch(() =>
		console.error(
			"The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`.",
		),
	);
}

export async function float32Param(x: number): Promise<void> {
	await invoke<void>("plugin:floats|float32-param", { x: x });
}
export async function float64Param(x: number): Promise<void> {
	await invoke<void>("plugin:floats|float64-param", { x: x });
}
export async function float32Result(): Promise<number> {
	const result = await invoke<number>("plugin:floats|float32-result");
	return result;
}
export async function float64Result(): Promise<number> {
	const result = await invoke<number>("plugin:floats|float64-result");
	return result;
}

