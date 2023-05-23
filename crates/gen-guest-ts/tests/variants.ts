export type Result<T, E> = { tag: 'ok', val: T } | { tag: 'err', val: E };
class Deserializer {
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

function de_varint(de, type) {
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
    return de_varint(de, 32)
}function deserializeU64(de) {
    return de_varint(de, 64)
}function deserializeS32(de) {
    const n = de_varint(de, 32)

    return Number(((n >> 1n) & 0xFFFFFFFFn) ^ (-((n & 0b1n) & 0xFFFFFFFFn)))
}function deserializeS64(de) {
    const n = de_varint(de, 64)

    return Number(((n >> 1n) & 0xFFFFFFFFFFFFFFFFn) ^ (-((n & 0b1n) & 0xFFFFFFFFFFFFFFFFn)))
}function deserializeF32(de) {
    const bytes = de.try_take_n(4);

    const buf = new ArrayBuffer(4);
    const view = new DataView(buf);

    bytes.forEach((v, i) => view.setUint8(i, v));

    return view.getFloat32(0, true);
}function deserializeF64(de) {
    const bytes = de.try_take_n(8);

    const buf = new ArrayBuffer(8);
    const view = new DataView(buf);

    bytes.forEach((v, i) => view.setUint8(i, v));

    return view.getFloat64(0, true);
}function deserializeString(de) {
    const sz = deserializeU64(de);

    let bytes = de.try_take_n(Number(sz));

    const decoder = new TextDecoder('utf-8');

    return decoder.decode(bytes);
}function deserializeBytes(de) {
    const sz = deserializeU64(de);

    let bytes = de.try_take_n(Number(sz));

    return bytes;
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
}function ser_varint(out, type, val) {
    for (let i = 0; i < varint_max(type); i++) {
        const buffer = new ArrayBuffer(type / 8);
        const view = new DataView(buffer);
        view.setInt16(0, Number(val), true);
        out[i] = view.getUint8(0);
        if (val < 128n) {
            return;
        }

        out[i] |= 0x80;
        val >>= 7n;
    }
}
function serializeBool(out, val) {
    out.push(val === true ? 1 : 0)
}function serializeU8(out, val) {
    return out.push(val)
}function serializeU32(out, val) {
    return ser_varint(out, 32, val)
}function serializeU64(out, val) {
    return ser_varint(out, 64, val)
}function serializeS32(out, val) {
    ser_varint(out, 32, BigInt((val << 1) ^ (val >> 31)))
}function serializeS64(out, val) {
    ser_varint(out, 64, BigInt((val << 1) ^ (val >> 63)))
}function serializeF32(out, val) {
    const buf = new Uint8Array(4);
    const view = new DataView(buf);

    view.setFloat32(0, val, true);

    out.push([...buf])
}function serializeF64(out, val) {
    const buf = new Uint8Array(8);
    const view = new DataView(buf);

    view.setFloat64(0, val, true);

    out.push([...buf])
}function serializeString(out, val) {
    serializeU64(out, val.length);

    const encoder = new TextEncoder();

    out.push(...encoder.encode(val))
}function serializeBytes(out, val) {
    serializeU64(out, val.length);
    out.push(...val)
}function serializeOption(out, inner, val) {
    serializeU8(out, !!val ? 1 : 0)
    inner(out, val)
}function serializeResult(out, ok, err, val) {
    switch (val.tag) {
        case 0:
            serializeU8(out, 0);
            return ok(out, val.val);
        case 1:
            serializeU8(out, 1);
            return err(out, val.val);
        default:
            throw new Error('Serialize bad result');
    }
}function deserializeE1(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0n:
                return "A"
            
                    default:
                        throw new Error("unknown enum case")
                }
        }function deserializeU1(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0n:
                return { tag: 0, value: deserializeU32(de) }
            case 1n:
                return { tag: 1, value: deserializeF32(de) }
            
                    default:
                        throw new Error("unknown variant case")
                }
        }function deserializeEmpty(de) {
            return {
                
            }
        }function deserializeV1(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0n:
                return { tag: 0, value: null }
            case 1n:
                return { tag: 1, value: deserializeU1(de) }
            case 2n:
                return { tag: 2, value: deserializeE1(de) }
            case 3n:
                return { tag: 3, value: deserializeString(de) }
            case 4n:
                return { tag: 4, value: deserializeEmpty(de) }
            case 5n:
                return { tag: 5, value: null }
            case 6n:
                return { tag: 6, value: deserializeU32(de) }
            
                    default:
                        throw new Error("unknown variant case")
                }
        }function deserializeCasts1(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0n:
                return { tag: 0, value: deserializeS32(de) }
            case 1n:
                return { tag: 1, value: deserializeF32(de) }
            
                    default:
                        throw new Error("unknown variant case")
                }
        }function deserializeCasts2(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0n:
                return { tag: 0, value: deserializeF64(de) }
            case 1n:
                return { tag: 1, value: deserializeF32(de) }
            
                    default:
                        throw new Error("unknown variant case")
                }
        }function deserializeCasts3(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0n:
                return { tag: 0, value: deserializeF64(de) }
            case 1n:
                return { tag: 1, value: deserializeU64(de) }
            
                    default:
                        throw new Error("unknown variant case")
                }
        }function deserializeCasts4(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0n:
                return { tag: 0, value: deserializeU32(de) }
            case 1n:
                return { tag: 1, value: deserializeS64(de) }
            
                    default:
                        throw new Error("unknown variant case")
                }
        }function deserializeCasts5(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0n:
                return { tag: 0, value: deserializeF32(de) }
            case 1n:
                return { tag: 1, value: deserializeS64(de) }
            
                    default:
                        throw new Error("unknown variant case")
                }
        }function deserializeCasts6(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0n:
                return { tag: 0, value: [deserializeF32(de), deserializeU32(de)] }
            case 1n:
                return { tag: 1, value: [deserializeU32(de), deserializeU32(de)] }
            
                    default:
                        throw new Error("unknown variant case")
                }
        }function deserializeMyErrno(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0n:
                return "Bad1"
            case 1n:
                return "Bad2"
            
                    default:
                        throw new Error("unknown enum case")
                }
        }function deserializeIsClone(de) {
            return {
                v1: deserializeV1(de)
            }
        }function serializeE1(out, val) {
                switch (val) {
                    case "A":
                    serializeU32(out, 0)
                    return
            
                    default:
                        throw new Error("unknown enum case")
                }
        }function serializeU1(out, val) {
                serializeU32(out, val.tag)

                switch (val.tag) {
                    case 0:
                    serializeU32(out, val.val)
                    return
                case 1:
                    serializeF32(out, val.val)
                    return
                
                    default:
                        throw new Error("unknown union case")
                }
        }function serializeEmpty(out, val) {
                
            }function serializeV1(out, val) {
                serializeU32(out, val.tag)

                switch (val.tag) {
                    case 0:
                    
                    return
                case 1:
                    serializeU1(out, val.val)
                    return
                case 2:
                    serializeE1(out, val.val)
                    return
                case 3:
                    serializeString(out, val.val)
                    return
                case 4:
                    serializeEmpty(out, val.val)
                    return
                case 5:
                    
                    return
                case 6:
                    serializeU32(out, val.val)
                    return
                
                    default:
                        throw new Error("unknown variant case")
                }
        }function serializeCasts1(out, val) {
                serializeU32(out, val.tag)

                switch (val.tag) {
                    case 0:
                    serializeS32(out, val.val)
                    return
                case 1:
                    serializeF32(out, val.val)
                    return
                
                    default:
                        throw new Error("unknown variant case")
                }
        }function serializeCasts2(out, val) {
                serializeU32(out, val.tag)

                switch (val.tag) {
                    case 0:
                    serializeF64(out, val.val)
                    return
                case 1:
                    serializeF32(out, val.val)
                    return
                
                    default:
                        throw new Error("unknown variant case")
                }
        }function serializeCasts3(out, val) {
                serializeU32(out, val.tag)

                switch (val.tag) {
                    case 0:
                    serializeF64(out, val.val)
                    return
                case 1:
                    serializeU64(out, val.val)
                    return
                
                    default:
                        throw new Error("unknown variant case")
                }
        }function serializeCasts4(out, val) {
                serializeU32(out, val.tag)

                switch (val.tag) {
                    case 0:
                    serializeU32(out, val.val)
                    return
                case 1:
                    serializeS64(out, val.val)
                    return
                
                    default:
                        throw new Error("unknown variant case")
                }
        }function serializeCasts5(out, val) {
                serializeU32(out, val.tag)

                switch (val.tag) {
                    case 0:
                    serializeF32(out, val.val)
                    return
                case 1:
                    serializeS64(out, val.val)
                    return
                
                    default:
                        throw new Error("unknown variant case")
                }
        }function serializeCasts6(out, val) {
                serializeU32(out, val.tag)

                switch (val.tag) {
                    case 0:
                    {serializeF32(out, val.val[0]);serializeU32(out, val.val[1])}
                    return
                case 1:
                    {serializeU32(out, val.val[0]);serializeU32(out, val.val[1])}
                    return
                
                    default:
                        throw new Error("unknown variant case")
                }
        }function serializeIsClone(out, val) {
                serializeV1(out, val.v1)
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


            
            export async function e1Arg (x: E1) : Promise<void> {
                const out = []
                serializeE1(out, x)
                
                 fetch('ipc://localhost/variants/e1_arg', { method: "POST", body: Uint8Array.from(out) }) 
            }
        
            
            export async function e1Result () : Promise<E1> {
                const out = []
                
                
                return fetch('ipc://localhost/variants/e1_result', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeE1(de)
                }) as Promise<E1>
            }
        
            
            export async function u1Arg (x: U1) : Promise<void> {
                const out = []
                serializeU1(out, x)
                
                 fetch('ipc://localhost/variants/u1_arg', { method: "POST", body: Uint8Array.from(out) }) 
            }
        
            
            export async function u1Result () : Promise<U1> {
                const out = []
                
                
                return fetch('ipc://localhost/variants/u1_result', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeU1(de)
                }) as Promise<U1>
            }
        
            
            export async function v1Arg (x: V1) : Promise<void> {
                const out = []
                serializeV1(out, x)
                
                 fetch('ipc://localhost/variants/v1_arg', { method: "POST", body: Uint8Array.from(out) }) 
            }
        
            
            export async function v1Result () : Promise<V1> {
                const out = []
                
                
                return fetch('ipc://localhost/variants/v1_result', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeV1(de)
                }) as Promise<V1>
            }
        
            
            export async function boolArg (x: boolean) : Promise<void> {
                const out = []
                serializeBool(out, x)
                
                 fetch('ipc://localhost/variants/bool_arg', { method: "POST", body: Uint8Array.from(out) }) 
            }
        
            
            export async function boolResult () : Promise<boolean> {
                const out = []
                
                
                return fetch('ipc://localhost/variants/bool_result', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeBool(de)
                }) as Promise<boolean>
            }
        
            
            export async function optionArg (a: boolean | null, b: [] | null, c: number | null, d: E1 | null, e: number | null, f: U1 | null, g: boolean | null | null) : Promise<void> {
                const out = []
                serializeOption(out, (v) => serializeBool(out, v), a);
serializeOption(out, (v) => {}, b);
serializeOption(out, (v) => serializeU32(out, v), c);
serializeOption(out, (v) => serializeE1(out, v), d);
serializeOption(out, (v) => serializeF32(out, v), e);
serializeOption(out, (v) => serializeU1(out, v), f);
serializeOption(out, (v) => serializeOption(out, (v) => serializeBool(out, v), v), g)
                
                 fetch('ipc://localhost/variants/option_arg', { method: "POST", body: Uint8Array.from(out) }) 
            }
        
            
            export async function optionResult () : Promise<[boolean | null, [] | null, number | null, E1 | null, number | null, U1 | null, boolean | null | null]> {
                const out = []
                
                
                return fetch('ipc://localhost/variants/option_result', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return [deserializeOption(de, (de) => deserializeBool(de)), deserializeOption(de, (de) => []), deserializeOption(de, (de) => deserializeU32(de)), deserializeOption(de, (de) => deserializeE1(de)), deserializeOption(de, (de) => deserializeF32(de)), deserializeOption(de, (de) => deserializeU1(de)), deserializeOption(de, (de) => deserializeOption(de, (de) => deserializeBool(de)))]
                }) as Promise<[boolean | null, [] | null, number | null, E1 | null, number | null, U1 | null, boolean | null | null]>
            }
        
            
            export async function casts (a: Casts1, b: Casts2, c: Casts3, d: Casts4, e: Casts5, f: Casts6) : Promise<[Casts1, Casts2, Casts3, Casts4, Casts5, Casts6]> {
                const out = []
                serializeCasts1(out, a);
serializeCasts2(out, b);
serializeCasts3(out, c);
serializeCasts4(out, d);
serializeCasts5(out, e);
serializeCasts6(out, f)
                
                return fetch('ipc://localhost/variants/casts', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return [deserializeCasts1(de), deserializeCasts2(de), deserializeCasts3(de), deserializeCasts4(de), deserializeCasts5(de), deserializeCasts6(de)]
                }) as Promise<[Casts1, Casts2, Casts3, Casts4, Casts5, Casts6]>
            }
        
            
            export async function resultArg (a: Result<null, null>, b: Result<null, E1>, c: Result<E1, null>, d: Result<[], []>, e: Result<number, V1>, f: Result<string, Uint8Array[]>) : Promise<void> {
                const out = []
                serializeResult(out, (v) => {}, (v) => {}, a);
serializeResult(out, (v) => {}, (v) => serializeE1(out, v), b);
serializeResult(out, (v) => serializeE1(out, v), (v) => {}, c);
serializeResult(out, (v) => {}, (v) => {}, d);
serializeResult(out, (v) => serializeU32(out, v), (v) => serializeV1(out, v), e);
serializeResult(out, (v) => serializeString(out, v), (v) => serializeList(out, (v) => serializeU8(out, v), v), f)
                
                 fetch('ipc://localhost/variants/result_arg', { method: "POST", body: Uint8Array.from(out) }) 
            }
        
            
            export async function resultResult () : Promise<[Result<null, null>, Result<null, E1>, Result<E1, null>, Result<[], []>, Result<number, V1>, Result<string, Uint8Array[]>]> {
                const out = []
                
                
                return fetch('ipc://localhost/variants/result_result', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return [deserializeResult(de, () => {}, () => {}), deserializeResult(de, () => {}, deserializeE1(de)), deserializeResult(de, deserializeE1(de), () => {}), deserializeResult(de, [], []), deserializeResult(de, deserializeU32(de), deserializeV1(de)), deserializeResult(de, deserializeString(de), deserializeBytes(de))]
                }) as Promise<[Result<null, null>, Result<null, E1>, Result<E1, null>, Result<[], []>, Result<number, V1>, Result<string, Uint8Array[]>]>
            }
        
            
            export async function returnResultSugar () : Promise<Result<number, MyErrno>> {
                const out = []
                
                
                return fetch('ipc://localhost/variants/return_result_sugar', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeResult(de, deserializeS32(de), deserializeMyErrno(de))
                }) as Promise<Result<number, MyErrno>>
            }
        
            
            export async function returnResultSugar2 () : Promise<Result<null, MyErrno>> {
                const out = []
                
                
                return fetch('ipc://localhost/variants/return_result_sugar2', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeResult(de, () => {}, deserializeMyErrno(de))
                }) as Promise<Result<null, MyErrno>>
            }
        
            
            export async function returnResultSugar3 () : Promise<Result<MyErrno, MyErrno>> {
                const out = []
                
                
                return fetch('ipc://localhost/variants/return_result_sugar3', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeResult(de, deserializeMyErrno(de), deserializeMyErrno(de))
                }) as Promise<Result<MyErrno, MyErrno>>
            }
        
            
            export async function returnResultSugar4 () : Promise<Result<[number, number], MyErrno>> {
                const out = []
                
                
                return fetch('ipc://localhost/variants/return_result_sugar4', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeResult(de, [deserializeS32(de), deserializeU32(de)], deserializeMyErrno(de))
                }) as Promise<Result<[number, number], MyErrno>>
            }
        
            
            export async function returnOptionSugar () : Promise<number | null> {
                const out = []
                
                
                return fetch('ipc://localhost/variants/return_option_sugar', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeOption(de, (de) => deserializeS32(de))
                }) as Promise<number | null>
            }
        
            
            export async function returnOptionSugar2 () : Promise<MyErrno | null> {
                const out = []
                
                
                return fetch('ipc://localhost/variants/return_option_sugar2', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeOption(de, (de) => deserializeMyErrno(de))
                }) as Promise<MyErrno | null>
            }
        
            
            export async function resultSimple () : Promise<Result<number, number>> {
                const out = []
                
                
                return fetch('ipc://localhost/variants/result_simple', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeResult(de, deserializeU32(de), deserializeS32(de))
                }) as Promise<Result<number, number>>
            }
        
            
            export async function isCloneArg (a: IsClone) : Promise<void> {
                const out = []
                serializeIsClone(out, a)
                
                 fetch('ipc://localhost/variants/is_clone_arg', { method: "POST", body: Uint8Array.from(out) }) 
            }
        
            
            export async function isCloneReturn () : Promise<IsClone> {
                const out = []
                
                
                return fetch('ipc://localhost/variants/is_clone_return', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeIsClone(de)
                }) as Promise<IsClone>
            }
        
            
            export async function returnNamedOption () : Promise<number | null> {
                const out = []
                
                
                return fetch('ipc://localhost/variants/return_named_option', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeOption(de, (de) => deserializeU8(de))
                }) as Promise<number | null>
            }
        
            
            export async function returnNamedResult () : Promise<Result<number, MyErrno>> {
                const out = []
                
                
                return fetch('ipc://localhost/variants/return_named_result', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeResult(de, deserializeU8(de), deserializeMyErrno(de))
                }) as Promise<Result<number, MyErrno>>
            }
        