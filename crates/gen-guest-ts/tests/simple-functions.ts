declare global {
	interface Window {
		__TAURI_INVOKE__<T>(
			cmd: string,
			args?: Record<string, unknown>,
		): Promise<T>;
	}
}
const invoke = window.__TAURI_INVOKE__;
export async function f1(): Promise<[]> {
	const result = await invoke<[]>("plugin:d52f0e93c1bb4daa|f1");
	return result;
}
export async function f2(a: number): Promise<[]> {
	const result = await invoke<[]>("plugin:d52f0e93c1bb4daa|f2", { a: a });
	return result;
}
export async function f3(a: number, b: number): Promise<[]> {
	const result = await invoke<[]>("plugin:d52f0e93c1bb4daa|f3", { a: a, b: b });
	return result;
}
export async function f4(): Promise<number> {
	const result = await invoke<number>("plugin:d52f0e93c1bb4daa|f4");
	return result;
}
export async function f5(): Promise<[number, number]> {
	const result = await invoke<[number, number]>("plugin:d52f0e93c1bb4daa|f5");
	return result;
}
export async function f6(
	a: number,
	b: number,
	c: number,
): Promise<[number, number, number]> {
	const result = await invoke<[number, number, number]>(
		"plugin:d52f0e93c1bb4daa|f6",
		{ a: a, b: b, c: c },
	);
	return result;
}

