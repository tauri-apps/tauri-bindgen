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
	invoke("plugin|chars:678374cfb5cdb2b5ba845e4b559f402a").catch(() =>
		console.error(
			"The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`.",
		),
	);
}

/**
 * A function that accepts a character
 */
export async function takeChar(x: string): Promise<void> {
	await invoke<void>("plugin:chars|take-char", { x: x });
}
/**
 * A function that returns a character
 */
export async function returnChar(): Promise<string> {
	const result = await invoke<string>("plugin:chars|return-char");
	return result;
}

