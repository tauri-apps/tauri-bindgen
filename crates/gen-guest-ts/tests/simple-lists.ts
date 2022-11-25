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
	invoke("plugin|simple_lists:d40a3203ef48115d7df3e6859a69ed77").catch(() =>
		console.error(
			"The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`.",
		),
	);
}

export async function simpleList1(l: Uint32Array): Promise<void> {
	await invoke<void>("plugin:simple_lists|simple-list1", { l: l });
}
export async function simpleList2(): Promise<Uint32Array> {
	const result = await invoke<Uint32Array>("plugin:simple_lists|simple-list2");
	return result;
}
export async function simpleList4(l: Uint32Array[]): Promise<Uint32Array[]> {
	const result = await invoke<Uint32Array[]>(
		"plugin:simple_lists|simple-list4",
		{ l: l },
	);
	return result;
}

