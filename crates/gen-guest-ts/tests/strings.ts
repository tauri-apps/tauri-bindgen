declare global {
	interface Window {
		__TAURI_INVOKE__<T>(
			cmd: string,
			args?: Record<string, unknown>,
		): Promise<T>;
	}
}
const invoke = window.__TAURI_INVOKE__;
export async function a(x: string): Promise<void> {
	await invoke<void>("plugin:16c3ebd2deefea81|a", { x: x });
}
export async function b(): Promise<string> {
	const result = await invoke<string>("plugin:16c3ebd2deefea81|b");
	return result;
}
export async function c(a: string, b: string): Promise<string> {
	const result = await invoke<string>("plugin:16c3ebd2deefea81|c", {
		a: a,
		b: b,
	});
	return result;
}

