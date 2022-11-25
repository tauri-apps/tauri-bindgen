declare global {
	interface Window {
		__TAURI_INVOKE__<T>(
			cmd: string,
			args?: Record<string, unknown>,
		): Promise<T>;
	}
}
const invoke = window.__TAURI_INVOKE__;
export async function mra(): Promise<void> {
	await invoke<void>("plugin:d238f57052cdcb90|mra");
}
export async function mrb(): Promise<void> {
	await invoke<void>("plugin:d238f57052cdcb90|mrb");
}
export async function mrc(): Promise<number> {
	const result = await invoke<number>("plugin:d238f57052cdcb90|mrc");
	return result;
}
export async function mrd(): Promise<number> {
	const result = await invoke<number>("plugin:d238f57052cdcb90|mrd");
	return result;
}
export async function mre(): Promise<[number, number]> {
	const result = await invoke<[number, number]>("plugin:d238f57052cdcb90|mre");
	return result;
}

