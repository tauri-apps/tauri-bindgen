declare global {
	interface Window {
		__TAURI_INVOKE__<T>(
			cmd: string,
			args?: Record<string, unknown>,
		): Promise<T>;
	}
}
const invoke = window.__TAURI_INVOKE__;
export async function a(x: string): Promise<[]> {
	const result = await invoke<[]>("plugin:4883b53925a5f618|a", { x: x });
	return result;
}
export async function b(): Promise<string> {
	const result = await invoke<string>("plugin:4883b53925a5f618|b");
	return result;
}
export async function c(a: string, b: string): Promise<string> {
	const result = await invoke<string>("plugin:4883b53925a5f618|c", {
		a: a,
		b: b,
	});
	return result;
}

