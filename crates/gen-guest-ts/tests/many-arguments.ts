declare global {
	interface Window {
		__TAURI_INVOKE__<T>(
			cmd: string,
			args?: Record<string, unknown>,
		): Promise<T>;
	}
}
const invoke = window.__TAURI_INVOKE__;
export interface BigStruct {
	a1: string;
	a2: string;
	a3: string;
	a4: string;
	a5: string;
	a6: string;
	a7: string;
	a8: string;
	a9: string;
	a10: string;
	a11: string;
	a12: string;
	a13: string;
	a14: string;
	a15: string;
	a16: string;
	a17: string;
	a18: string;
	a19: string;
	a20: string;
}
export async function manyArgs(
	a1: bigint,
	a2: bigint,
	a3: bigint,
	a4: bigint,
	a5: bigint,
	a6: bigint,
	a7: bigint,
	a8: bigint,
	a9: bigint,
	a10: bigint,
	a11: bigint,
	a12: bigint,
	a13: bigint,
	a14: bigint,
	a15: bigint,
	a16: bigint,
): Promise<void> {
	await invoke<void>("plugin:92d5120c|many-args", {
		a1: a1,
		a2: a2,
		a3: a3,
		a4: a4,
		a5: a5,
		a6: a6,
		a7: a7,
		a8: a8,
		a9: a9,
		a10: a10,
		a11: a11,
		a12: a12,
		a13: a13,
		a14: a14,
		a15: a15,
		a16: a16,
	});
}
export async function bigArgument(x: BigStruct): Promise<void> {
	await invoke<void>("plugin:92d5120c|big-argument", { x: x });
}

