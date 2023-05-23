export type Result<T, E> = { tag: 'ok', val: T } | { tag: 'err', val: E };
export class Deserializer {
    source
    offset
    
    constructor(bytes) {
        this.source = bytes
        this.offset = 0
    }

    pop() {
        return this.source[this.offset++]
    }

    try_take_n(len) {
        const out = this.source.slice(this.offset, this.offset + len)
        this.offset += len
        return out
    }
}
function varint_max(type) {
    const BITS_PER_BYTE = 8;
    const BITS_PER_VARINT_BYTE = 7;

    const bits = type * BITS_PER_BYTE;

    const roundup_bits = bits + (BITS_PER_BYTE - 1);

    return Math.floor(roundup_bits / BITS_PER_VARINT_BYTE);
}

function max_of_last_byte(type) {
    let extra_bits = type % 7;
    return (1 << extra_bits) - 1;
}

function try_take_varint(de, type) {
    let out = 0n;

    for (let i = 0; i < varint_max(type); i++) {
        const val = de.pop();
        const carry = BigInt(val & 0x7F);
        out |= carry << (7n * BigInt(i));

        if ((val & 0x80) === 0) {
            if (i === varint_max(type) - 1 && val > max_of_last_byte(type)) {
                throw new Error('deserialize bad variant')
            } else {
                return out
            }
        }
    }

    throw new Error('deserialize bad variant')
}function deserializeBool(de) {
    const val = de.pop();

    return val != 0
}function deserializeU8(de) {
    return de.pop()
}function deserializeU32(de) {
    return try_take_varint(de, 32)
}function deserializeU64(de) {
    return try_take_varint(de, 64)
}function deserializeS32(de) {
    const n = try_take_varint(de, 32)

    return Number(((n >> 1n) & 0xFFFFFFFFn) ^ (-((n & 0b1n) & 0xFFFFFFFFn)))
}function deserializeS64(de) {
    const n = try_take_varint(de, u64)

    return Number(((n >> 1n) & 0xFFFFFFFFFFFFFFFFn) ^ (-((n & 0b1n) & 0xFFFFFFFFFFFFFFFFn)))
}function deserializeF32(de) {
    const bytes = de.try_take_n(4);

    const buf = new ArrayBuffer(4);
    const view = new DataView(buf);

    bytes.reverse().forEach((v, i) => view.setUint8(i, v));

    return view.getFloat32(0);
}function deserializeF64(de) {
    const bytes = de.try_take_n(8);

    const buf = new ArrayBuffer(8);
    const view = new DataView(buf);

    bytes.reverse().forEach((v, i) => view.setUint8(i, v));

    return view.getFloat64(0);
}function deserializeString(de) {
    const sz = deserializeU64(de);

    let bytes = de.try_take_n(Number(sz));

    const decoder = new TextDecoder('utf-8');

    return decoder.decode(bytes);
}function deserializeBytes(de) {
    const sz = deserializeU64(de);

    let bytes = de.try_take_n(Number(sz));

    const decoder = new TextDecoder('utf-8');

    return decoder.decode(bytes);
}function deserializeOption(de, inner) {
    const disc = de.pop()

    switch (disc) {
        case 0:
            return null
        case 1: 
            return inner(de)
        default:
            throw new Error('Deserialize bad option')
    }
}function deserializeResult(de, ok, err) {
    const disc = de.pop()

    switch (disc) {
        case 0:
            return ok(de)
        case 1: 
            return err(de)
        default:
            throw new Error('Deserialize bad result')
    }
}function deserializeE1(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0:
                return "A"
            
                    default:
                        throw new Error("unknown enum case")
                }
        }function deserializeU1(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0:
                return deserializeU32(de)
            case 1:
                return deserializeF32(de)
            
                    default:
                        throw new Error("unknown union case")
                }
        }function deserializeEmpty(de) {
            return {
                
            }
        }function deserializeV1(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0:
                return { tag: 0, value: null }
            case 1:
                return { tag: 1, value: deserializeU1(de) }
            case 2:
                return { tag: 2, value: deserializeE1(de) }
            case 3:
                return { tag: 3, value: deserializeString(de) }
            case 4:
                return { tag: 4, value: deserializeEmpty(de) }
            case 5:
                return { tag: 5, value: null }
            case 6:
                return { tag: 6, value: deserializeU32(de) }
            
                    default:
                        throw new Error("unknown variant case")
                }
        }function deserializeCasts1(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0:
                return { tag: 0, value: deserializeS32(de) }
            case 1:
                return { tag: 1, value: deserializeF32(de) }
            
                    default:
                        throw new Error("unknown variant case")
                }
        }function deserializeCasts2(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0:
                return { tag: 0, value: deserializeF64(de) }
            case 1:
                return { tag: 1, value: deserializeF32(de) }
            
                    default:
                        throw new Error("unknown variant case")
                }
        }function deserializeCasts3(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0:
                return { tag: 0, value: deserializeF64(de) }
            case 1:
                return { tag: 1, value: deserializeU64(de) }
            
                    default:
                        throw new Error("unknown variant case")
                }
        }function deserializeCasts4(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0:
                return { tag: 0, value: deserializeU32(de) }
            case 1:
                return { tag: 1, value: deserializeS64(de) }
            
                    default:
                        throw new Error("unknown variant case")
                }
        }function deserializeCasts5(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0:
                return { tag: 0, value: deserializeF32(de) }
            case 1:
                return { tag: 1, value: deserializeS64(de) }
            
                    default:
                        throw new Error("unknown variant case")
                }
        }function deserializeCasts6(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0:
                return { tag: 0, value: [deserializeF32(de), deserializeU32(de)] }
            case 1:
                return { tag: 1, value: [deserializeU32(de), deserializeU32(de)] }
            
                    default:
                        throw new Error("unknown variant case")
                }
        }function deserializeMyErrno(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0:
                return "Bad1"
            case 1:
                return "Bad2"
            
                    default:
                        throw new Error("unknown enum case")
                }
        }function deserializeIsClone(de) {
            return {
                v1: deserializeV1(de)
            }
        }

export enum E1 { 
A,
 }

export type U1 = 
number
 | 
number
;

export interface Empty {  }

export interface V1A { tag: 0 }

export interface V1B { tag: 1, value: U1 }

export interface V1C { tag: 2, value: E1 }

export interface V1D { tag: 3, value: string }

export interface V1E { tag: 4, value: Empty }

export interface V1F { tag: 5 }

export interface V1G { tag: 6, value: number }


export type V1 = 
V1A | 
V1B | 
V1C | 
V1D | 
V1E | 
V1F | 
V1G

export interface Casts1A { tag: 0, value: number }

export interface Casts1B { tag: 1, value: number }


export type Casts1 = 
Casts1A | 
Casts1B

export interface Casts2A { tag: 0, value: number }

export interface Casts2B { tag: 1, value: number }


export type Casts2 = 
Casts2A | 
Casts2B

export interface Casts3A { tag: 0, value: number }

export interface Casts3B { tag: 1, value: bigint }


export type Casts3 = 
Casts3A | 
Casts3B

export interface Casts4A { tag: 0, value: number }

export interface Casts4B { tag: 1, value: bigint }


export type Casts4 = 
Casts4A | 
Casts4B

export interface Casts5A { tag: 0, value: number }

export interface Casts5B { tag: 1, value: bigint }


export type Casts5 = 
Casts5A | 
Casts5B

export interface Casts6A { tag: 0, value: [number, number] }

export interface Casts6B { tag: 1, value: [number, number] }


export type Casts6 = 
Casts6A | 
Casts6B

export enum MyErrno { 
Bad1,

Bad2,
 }

export interface IsClone { 
v1: V1,
 }


            
            export async function e1Arg (x: E1)  {
                return fetch('ipc://localhost/variants/e1_arg', { method: "POST", body: JSON.stringify([x]) })
            }
        
            
            export async function e1Result () : Promise<E1> {
                return fetch('ipc://localhost/variants/e1_result', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeE1(de)
                })
            }
        
            
            export async function u1Arg (x: U1)  {
                return fetch('ipc://localhost/variants/u1_arg', { method: "POST", body: JSON.stringify([x]) })
            }
        
            
            export async function u1Result () : Promise<U1> {
                return fetch('ipc://localhost/variants/u1_result', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeU1(de)
                })
            }
        
            
            export async function v1Arg (x: V1)  {
                return fetch('ipc://localhost/variants/v1_arg', { method: "POST", body: JSON.stringify([x]) })
            }
        
            
            export async function v1Result () : Promise<V1> {
                return fetch('ipc://localhost/variants/v1_result', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeV1(de)
                })
            }
        
            
            export async function boolArg (x: boolean)  {
                return fetch('ipc://localhost/variants/bool_arg', { method: "POST", body: JSON.stringify([x]) })
            }
        
            
            export async function boolResult () : Promise<boolean> {
                return fetch('ipc://localhost/variants/bool_result', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeBoolean(de)
                })
            }
        
            
            export async function optionArg (a: boolean | null, b: [] | null, c: number | null, d: E1 | null, e: number | null, f: U1 | null, g: boolean | null | null)  {
                return fetch('ipc://localhost/variants/option_arg', { method: "POST", body: JSON.stringify([a, b, c, d, e, f, g]) })
            }
        
            
            export async function optionResult () : Promise<[boolean | null, [] | null, number | null, E1 | null, number | null, U1 | null, boolean | null | null]> {
                return fetch('ipc://localhost/variants/option_result', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return [deserializeOption(de, (de) => deserializeBoolean(de)), deserializeOption(de, (de) => []), deserializeOption(de, (de) => deserializeU32(de)), deserializeOption(de, (de) => deserializeE1(de)), deserializeOption(de, (de) => deserializeF32(de)), deserializeOption(de, (de) => deserializeU1(de)), deserializeOption(de, (de) => deserializeOption(de, (de) => deserializeBoolean(de)))]
                })
            }
        
            
            export async function casts (a: Casts1, b: Casts2, c: Casts3, d: Casts4, e: Casts5, f: Casts6) : Promise<[Casts1, Casts2, Casts3, Casts4, Casts5, Casts6]> {
                return fetch('ipc://localhost/variants/casts', { method: "POST", body: JSON.stringify([a, b, c, d, e, f]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return [deserializeCasts1(de), deserializeCasts2(de), deserializeCasts3(de), deserializeCasts4(de), deserializeCasts5(de), deserializeCasts6(de)]
                })
            }
        
            
            export async function resultArg (a: Result<_, _>, b: Result<_, E1>, c: Result<E1, _>, d: Result<[], []>, e: Result<number, V1>, f: Result<string, Uint8Array[]>)  {
                return fetch('ipc://localhost/variants/result_arg', { method: "POST", body: JSON.stringify([a, b, c, d, e, f]) })
            }
        
            
            export async function resultResult () : Promise<[Result<_, _>, Result<_, E1>, Result<E1, _>, Result<[], []>, Result<number, V1>, Result<string, Uint8Array[]>]> {
                return fetch('ipc://localhost/variants/result_result', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return [deserializeResult(de, () => {}, () => {}), deserializeResult(de, () => {}, deserializeE1(de)), deserializeResult(de, deserializeE1(de), () => {}), deserializeResult(de, [], []), deserializeResult(de, deserializeU32(de), deserializeV1(de)), deserializeResult(de, deserializeString(de), deserializeBytes(de))]
                })
            }
        
            
            export async function returnResultSugar () : Promise<Result<number, MyErrno>> {
                return fetch('ipc://localhost/variants/return_result_sugar', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeResult(de, deserializeS32(de), deserializeMyErrno(de))
                })
            }
        
            
            export async function returnResultSugar2 () : Promise<Result<_, MyErrno>> {
                return fetch('ipc://localhost/variants/return_result_sugar2', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeResult(de, () => {}, deserializeMyErrno(de))
                })
            }
        
            
            export async function returnResultSugar3 () : Promise<Result<MyErrno, MyErrno>> {
                return fetch('ipc://localhost/variants/return_result_sugar3', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeResult(de, deserializeMyErrno(de), deserializeMyErrno(de))
                })
            }
        
            
            export async function returnResultSugar4 () : Promise<Result<[number, number], MyErrno>> {
                return fetch('ipc://localhost/variants/return_result_sugar4', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeResult(de, [deserializeS32(de), deserializeU32(de)], deserializeMyErrno(de))
                })
            }
        
            
            export async function returnOptionSugar () : Promise<number | null> {
                return fetch('ipc://localhost/variants/return_option_sugar', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeOption(de, (de) => deserializeS32(de))
                })
            }
        
            
            export async function returnOptionSugar2 () : Promise<MyErrno | null> {
                return fetch('ipc://localhost/variants/return_option_sugar2', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeOption(de, (de) => deserializeMyErrno(de))
                })
            }
        
            
            export async function resultSimple () : Promise<Result<number, number>> {
                return fetch('ipc://localhost/variants/result_simple', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeResult(de, deserializeU32(de), deserializeS32(de))
                })
            }
        
            
            export async function isCloneArg (a: IsClone)  {
                return fetch('ipc://localhost/variants/is_clone_arg', { method: "POST", body: JSON.stringify([a]) })
            }
        
            
            export async function isCloneReturn () : Promise<IsClone> {
                return fetch('ipc://localhost/variants/is_clone_return', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeIsClone(de)
                })
            }
        
            
            export async function returnNamedOption () : Promise<number | null> {
                return fetch('ipc://localhost/variants/return_named_option', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeOption(de, (de) => deserializeU8(de))
                })
            }
        
            
            export async function returnNamedResult () : Promise<Result<number, MyErrno>> {
                return fetch('ipc://localhost/variants/return_named_result', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeResult(de, deserializeU8(de), deserializeMyErrno(de))
                })
            }
        