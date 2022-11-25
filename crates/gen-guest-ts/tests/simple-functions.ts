declare global {
	interface Window {
		__TAURI_INVOKE__<T>(
			cmd: string,
			args?: Record<string, unknown>,
		): Promise<T>;
	}
}
const invoke = window.__TAURI_INVOKE__;
export async function f1(): Promise<void> {
	await invoke<void>("plugin:ebb2d6f0|f1");
}
export async function f2(a: number): Promise<void> {
	await invoke<void>("plugin:ebb2d6f0|f2", { a: a });
}
export async function f3(a: number, b: number): Promise<void> {
	await invoke<void>("plugin:ebb2d6f0|f3", { a: a, b: b });
}
export async function f4(): Promise<number> {
	const result = await invoke<number>("plugin:ebb2d6f0|f4");
	return result;
}
export async function f5(): Promise<[number, number]> {
	const result = await invoke<[number, number]>("plugin:ebb2d6f0|f5");
	return result;
}
export async function f6(
	a: number,
	b: number,
	c: number,
): Promise<[number, number, number]> {
	const result = await invoke<[number, number, number]>("plugin:ebb2d6f0|f6", {
		a: a,
		b: b,
		c: c,
	});
	return result;
}

