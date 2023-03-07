/**
 * @param {Empty} x
 * @returns {Promise<Empty>}
 */
export async function empty(x) {}

/**
 * @param {Scalars} val
 * @returns {Promise<Scalars>}
 */
export async function record_scalars(val) {}

/**
 * @param {ReallyFlags} val
 * @returns {Promise<ReallyFlags>}
 */
export async function record_really_flags(val) {}

/**
 * @param {Aggregates} val
 * @returns {Promise<Aggregates>}
 */
export async function record_aggregates(val) {}

/**
 * @param {Flag1} x
 * @returns {Promise<Flag1>}
 */
export async function flag1(x) {}

/**
 * @param {Flag2} x
 * @returns {Promise<Flag2>}
 */
export async function flag2(x) {}

/**
 * @param {Flag4} x
 * @returns {Promise<Flag4>}
 */
export async function flag4(x) {}

/**
 * @param {Flag8} x
 * @returns {Promise<Flag8>}
 */
export async function flag8(x) {}

/**
 * @param {Flag16} x
 * @returns {Promise<Flag16>}
 */
export async function flag16(x) {}

/**
 * @param {Flag32} x
 * @returns {Promise<Flag32>}
 */
export async function flag32(x) {}

/**
 * @param {Flag64} x
 * @returns {Promise<Flag64>}
 */
export async function flag64(x) {}

/**
 * @param {number} x
 * @returns {Promise<number>}
 */
export async function float32(x) {}

/**
 * @param {number} x
 * @returns {Promise<number>}
 */
export async function float64(x) {}

/**
 * @param {number} x
 * @returns {Promise<number>}
 */
export async function u8(x) {}

/**
 * @param {number} x
 * @returns {Promise<number>}
 */
export async function s8(x) {}

/**
 * @param {number} x
 * @returns {Promise<number>}
 */
export async function u16(x) {}

/**
 * @param {number} x
 * @returns {Promise<number>}
 */
export async function s16(x) {}

/**
 * @param {number} x
 * @returns {Promise<number>}
 */
export async function u32(x) {}

/**
 * @param {number} x
 * @returns {Promise<number>}
 */
export async function s32(x) {}

/**
 * @param {bigint} x
 * @returns {Promise<bigint>}
 */
export async function u64(x) {}

/**
 * @param {bigint} x
 * @returns {Promise<bigint>}
 */
export async function s64(x) {}

/**
 * @param {Uint8Array[]} x
 * @returns {Promise<Uint8Array[]>}
 */
export async function list_u8(x) {}

/**
 * @param {Uint16Array[]} x
 * @returns {Promise<Uint16Array[]>}
 */
export async function list_u16(x) {}

/**
 * @param {Uint32Array[]} x
 * @returns {Promise<Uint32Array[]>}
 */
export async function list_u32(x) {}

/**
 * @param {BigUint64Array[]} x
 * @returns {Promise<BigUint64Array[]>}
 */
export async function list_u64(x) {}

/**
 * @param {Int8Array[]} x
 * @returns {Promise<Int8Array[]>}
 */
export async function list_s8(x) {}

/**
 * @param {Int16Array[]} x
 * @returns {Promise<Int16Array[]>}
 */
export async function list_s16(x) {}

/**
 * @param {Int32Array[]} x
 * @returns {Promise<Int32Array[]>}
 */
export async function list_s32(x) {}

/**
 * @param {BigInt64Array[]} x
 * @returns {Promise<BigInt64Array[]>}
 */
export async function list_s64(x) {}

/**
 * @param {Float32Array[]} x
 * @returns {Promise<Float32Array[]>}
 */
export async function list_float32(x) {}

/**
 * @param {Float64Array[]} x
 * @returns {Promise<Float64Array[]>}
 */
export async function list_float64(x) {}

/**
 * @param {[number, number][]} x
 * @returns {Promise<[number, number][]>}
 */
export async function tuple_list(x) {}

/**
 * @param {string[]} a
 * @returns {Promise<string[]>}
 */
export async function string_list(a) {}

/**
 * @param {[number, string][]} x
 * @returns {Promise<[number, string][]>}
 */
export async function tuple_string_list(x) {}

/**
 * @param {SomeRecord[]} x
 * @returns {Promise<OtherRecord[]>}
 */
export async function record_list(x) {}

/**
 * @param {AllIntegers} x
 * @returns {Promise<AllIntegers>}
 */
export async function all_integers(x) {}

/**
 * @param {AllFloats} x
 * @returns {Promise<AllFloats>}
 */
export async function all_floats(x) {}

/**
 * @param {AllText} x
 * @returns {Promise<AllText>}
 */
export async function all_text(x) {}

/**
 * @param {E1} x
 * @returns {Promise<E1>}
 */
export async function e1(x) {}

/**
 * @param {V1} x
 * @returns {Promise<V1>}
 */
export async function v1(x) {}

/**
 * @param {boolean | null} a
 * @param {[] | null} b
 * @param {number | null} c
 * @param {E1 | null} d
 * @param {number | null} e
 * @param {U1 | null} f
 * @param {boolean | null | null} g
 * @returns {Promise<[boolean | null, [] | null, number | null, E1 | null, number | null, U1 | null, boolean | null | null]>}
 */
export async function options(a, b, c, d, e, f, g) {}

/**
 * @param {Result<_, _>} a
 * @param {Result<_, E1>} b
 * @param {Result<E1, _>} c
 * @param {Result<[], []>} d
 * @param {Result<number, V1>} e
 * @param {Result<string, Uint8Array[]>} f
 * @returns {Promise<[Result<_, _>, Result<_, E1>, Result<E1, _>, Result<[], []>, Result<number, V1>, Result<string, Uint8Array[]>]>}
 */
export async function results(a, b, c, d, e, f) {}

/**
 * @param {DuplicateTypes} a
 * @returns {Promise<DuplicateTypes>}
 */
export async function duplicate_types(a) {}
