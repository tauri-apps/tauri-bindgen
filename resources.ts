declare global {
	interface Window {
		__TAURI_INVOKE__<T>(
			cmd: string,
			args?: Record<string, unknown>,
		): Promise<T>;
	}
}
const invoke = window.__TAURI_INVOKE__;
export class A {
	#id: number;
	constructor(id: number) {
		this.#id = id;
	}
}
export class B {
	#id: number;
	constructor(id: number) {
		this.#id = id;
	}
}
export class C {
	#id: number;
	constructor(id: number) {
		this.#id = id;
	}
	async foo(): Promise<[]> {
		const result = await invoke<[]>("plugin:7bdaea4b69388cc4|foo", {
			__id: this.#id,
		});
		return result;
	}
	static async bar(name: string): Promise<string> {
		const result = await invoke<string>("plugin:7bdaea4b69388cc4|bar", {
			name: name,
		});
		return result;
	}
	static async fiz(): Promise<string> {
		const result = await invoke<string>("plugin:7bdaea4b69388cc4|fiz");
		return result;
	}
}

