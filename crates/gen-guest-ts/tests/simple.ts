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
	const result = await invoke<[]>("plugin:e42871a1699d9d60|f1");
	return result;
}
export async function f2(a: number): Promise<[]> {
	const result = await invoke<[]>("plugin:e42871a1699d9d60|f2", { a: a });
	return result;
}
export async function f3(a: number, b: number): Promise<[]> {
	const result = await invoke<[]>("plugin:e42871a1699d9d60|f3", { a: a, b: b });
	return result;
}
export async function f4(): Promise<number> {
	const result = await invoke<number>("plugin:e42871a1699d9d60|f4");
	return result;
}
export async function f5(): Promise<[number, number]> {
	const result = await invoke<[number, number]>("plugin:e42871a1699d9d60|f5");
	return result;
}
export async function f6(
	a: number,
	b: number,
	c: number,
): Promise<[number, number, number]> {
	const result = await invoke<[number, number, number]>(
		"plugin:e42871a1699d9d60|f6",
		{ a: a, b: b, c: c },
	);
	return result;
}

