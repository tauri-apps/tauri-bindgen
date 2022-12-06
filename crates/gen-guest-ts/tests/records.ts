declare global {
	interface Window {
		__TAURI_INVOKE__<T>(
			cmd: string,
			args?: Record<string, unknown>,
		): Promise<T>;
	}
}
const invoke = window.__TAURI_INVOKE__;
export interface Empty {}
/**
 * A record containing two scalar fields that both have the same type
 */
export interface Scalars {
	/**
	 * The first field, named a
	 */
	a: number;
	/**
	 * The second field, named b
	 */
	b: number;
}
/**
 * A record that is really just flags All of the fields are bool
 */
export interface ReallyFlags {
	a: boolean;
	b: boolean;
	c: boolean;
	d: boolean;
	e: boolean;
	f: boolean;
	g: boolean;
	h: boolean;
	i: boolean;
}
export interface Aggregates {
	a: Scalars;
	b: number;
	c: Empty;
	d: string;
	e: ReallyFlags;
}
export type TupleTypedef = [number];
export type IntTypedef = number;
export type TupleTypedef2 = [IntTypedef];
export async function tupleArg(x: [string, number]): Promise<[]> {
	const result = await invoke<[]>("plugin:ac98167d7d43eb21|tuple-arg", {
		x: x,
	});
	return result;
}
export async function tupleResult(): Promise<[string, number]> {
	const result = await invoke<[string, number]>(
		"plugin:ac98167d7d43eb21|tuple-result",
	);
	return result;
}
export async function emptyArg(x: Empty): Promise<[]> {
	const result = await invoke<[]>("plugin:ac98167d7d43eb21|empty-arg", {
		x: x,
	});
	return result;
}
export async function emptyResult(): Promise<Empty> {
	const result = await invoke<Empty>("plugin:ac98167d7d43eb21|empty-result");
	return result;
}
export async function scalarArg(x: Scalars): Promise<[]> {
	const result = await invoke<[]>("plugin:ac98167d7d43eb21|scalar-arg", {
		x: x,
	});
	return result;
}
export async function scalarResult(): Promise<Scalars> {
	const result = await invoke<Scalars>("plugin:ac98167d7d43eb21|scalar-result");
	return result;
}
export async function flagsArg(x: ReallyFlags): Promise<[]> {
	const result = await invoke<[]>("plugin:ac98167d7d43eb21|flags-arg", {
		x: x,
	});
	return result;
}
export async function flagsResult(): Promise<ReallyFlags> {
	const result = await invoke<ReallyFlags>(
		"plugin:ac98167d7d43eb21|flags-result",
	);
	return result;
}
export async function aggregateArg(x: Aggregates): Promise<[]> {
	const result = await invoke<[]>("plugin:ac98167d7d43eb21|aggregate-arg", {
		x: x,
	});
	return result;
}
export async function aggregateResult(): Promise<Aggregates> {
	const result = await invoke<Aggregates>(
		"plugin:ac98167d7d43eb21|aggregate-result",
	);
	return result;
}
export async function typedefInout(e: TupleTypedef2): Promise<number> {
	const result = await invoke<number>("plugin:ac98167d7d43eb21|typedef-inout", {
		e: e,
	});
	return result;
}

