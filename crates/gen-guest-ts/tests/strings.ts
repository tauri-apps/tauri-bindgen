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
	invoke("plugin|strings:16c3ebd2deefea81065e2001501951a6").catch(() =>
		console.error(
			"The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`.",
		),
	);
}

export async function a(x: string): Promise<void> {
	await invoke<void>("plugin:strings|a", { x: x });
}
export async function b(): Promise<string> {
	const result = await invoke<string>("plugin:strings|b");
	return result;
}
export async function c(a: string, b: string): Promise<string> {
	const result = await invoke<string>("plugin:strings|c", { a: a, b: b });
	return result;
}

