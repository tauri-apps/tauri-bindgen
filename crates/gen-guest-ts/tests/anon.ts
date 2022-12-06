declare global {
	interface Window {
		__TAURI_INVOKE__<T>(
			cmd: string,
			args?: Record<string, unknown>,
		): Promise<T>;
	}
}
const invoke = window.__TAURI_INVOKE__;
export type Error = "success" | "failure";
export async function optionTest(): Promise<string | null> {
	const result = await invoke<string | null>(
		"plugin:9f005dd416978e86|option-test",
	);
	return result;
}

