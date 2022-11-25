declare global {
	interface Window {
		__TAURI_INVOKE__<T>(
			cmd: string,
			args?: Record<string, unknown>,
		): Promise<T>;
	}
}
const invoke = window.__TAURI_INVOKE__;
export async function simpleList1(l: Uint32Array): Promise<void> {
	await invoke<void>("plugin:d40a3203ef48115d|simple-list1", { l: l });
}
export async function simpleList2(): Promise<Uint32Array> {
	const result = await invoke<Uint32Array>(
		"plugin:d40a3203ef48115d|simple-list2",
	);
	return result;
}
export async function simpleList4(l: Uint32Array[]): Promise<Uint32Array[]> {
	const result = await invoke<Uint32Array[]>(
		"plugin:d40a3203ef48115d|simple-list4",
		{ l: l },
	);
	return result;
}

