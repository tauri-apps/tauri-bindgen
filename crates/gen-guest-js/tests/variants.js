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
        const buffer = new Uint8Array(type / 8);
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

            /**
* @param {E1} x 
*/
            export async function e1Arg (x) {
                const out = []
                serializeE1(out, x)

                return fetch('ipc://localhost/variants/e1_arg', { method: "POST", body: Uint8Array.from(out) })
            }
        
            /**
* @returns {Promise<E1>} 
*/
            export async function e1Result () {
                const out = []
                

                return fetch('ipc://localhost/variants/e1_result', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeE1(de)
                })
            }
        
            /**
* @param {U1} x 
*/
            export async function u1Arg (x) {
                const out = []
                serializeU1(out, x)

                return fetch('ipc://localhost/variants/u1_arg', { method: "POST", body: Uint8Array.from(out) })
            }
        
            /**
* @returns {Promise<U1>} 
*/
            export async function u1Result () {
                const out = []
                

                return fetch('ipc://localhost/variants/u1_result', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeU1(de)
                })
            }
        
            /**
* @param {V1} x 
*/
            export async function v1Arg (x) {
                const out = []
                serializeV1(out, x)

                return fetch('ipc://localhost/variants/v1_arg', { method: "POST", body: Uint8Array.from(out) })
            }
        
            /**
* @returns {Promise<V1>} 
*/
            export async function v1Result () {
                const out = []
                

                return fetch('ipc://localhost/variants/v1_result', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeV1(de)
                })
            }
        
            /**
* @param {boolean} x 
*/
            export async function boolArg (x) {
                const out = []
                serializeBool(out, x)

                return fetch('ipc://localhost/variants/bool_arg', { method: "POST", body: Uint8Array.from(out) })
            }
        
            /**
* @returns {Promise<boolean>} 
*/
            export async function boolResult () {
                const out = []
                

                return fetch('ipc://localhost/variants/bool_result', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeBool(de)
                })
            }
        
            /**
* @param {boolean | null} a 
* @param {[] | null} b 
* @param {number | null} c 
* @param {E1 | null} d 
* @param {number | null} e 
* @param {U1 | null} f 
* @param {boolean | null | null} g 
*/
            export async function optionArg (a, b, c, d, e, f, g) {
                const out = []
                serializeOption(out, (v) => serializeBool(out, v), a);
serializeOption(out, (v) => {}, b);
serializeOption(out, (v) => serializeU32(out, v), c);
serializeOption(out, (v) => serializeE1(out, v), d);
serializeOption(out, (v) => serializeF32(out, v), e);
serializeOption(out, (v) => serializeU1(out, v), f);
serializeOption(out, (v) => serializeOption(out, (v) => serializeBool(out, v), v), g)

                return fetch('ipc://localhost/variants/option_arg', { method: "POST", body: Uint8Array.from(out) })
            }
        
            /**
* @returns {Promise<[boolean | null, [] | null, number | null, E1 | null, number | null, U1 | null, boolean | null | null]>} 
*/
            export async function optionResult () {
                const out = []
                

                return fetch('ipc://localhost/variants/option_result', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return [deserializeOption(de, (de) => deserializeBool(de)), deserializeOption(de, (de) => []), deserializeOption(de, (de) => deserializeU32(de)), deserializeOption(de, (de) => deserializeE1(de)), deserializeOption(de, (de) => deserializeF32(de)), deserializeOption(de, (de) => deserializeU1(de)), deserializeOption(de, (de) => deserializeOption(de, (de) => deserializeBool(de)))]
                })
            }
        
            /**
* @param {Casts1} a 
* @param {Casts2} b 
* @param {Casts3} c 
* @param {Casts4} d 
* @param {Casts5} e 
* @param {Casts6} f 
* @returns {Promise<[Casts1, Casts2, Casts3, Casts4, Casts5, Casts6]>} 
*/
            export async function casts (a, b, c, d, e, f) {
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
                })
            }
        
            /**
* @param {Result<_, _>} a 
* @param {Result<_, E1>} b 
* @param {Result<E1, _>} c 
* @param {Result<[], []>} d 
* @param {Result<number, V1>} e 
* @param {Result<string, Uint8Array[]>} f 
*/
            export async function resultArg (a, b, c, d, e, f) {
                const out = []
                serializeResult(out, (v) => {}, (v) => {}, a);
serializeResult(out, (v) => {}, (v) => serializeE1(out, v), b);
serializeResult(out, (v) => serializeE1(out, v), (v) => {}, c);
serializeResult(out, (v) => {}, (v) => {}, d);
serializeResult(out, (v) => serializeU32(out, v), (v) => serializeV1(out, v), e);
serializeResult(out, (v) => serializeString(out, v), (v) => serializeList(out, (v) => serializeU8(out, v), v), f)

                return fetch('ipc://localhost/variants/result_arg', { method: "POST", body: Uint8Array.from(out) })
            }
        
            /**
* @returns {Promise<[Result<_, _>, Result<_, E1>, Result<E1, _>, Result<[], []>, Result<number, V1>, Result<string, Uint8Array[]>]>} 
*/
            export async function resultResult () {
                const out = []
                

                return fetch('ipc://localhost/variants/result_result', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return [deserializeResult(de, () => {}, () => {}), deserializeResult(de, () => {}, deserializeE1(de)), deserializeResult(de, deserializeE1(de), () => {}), deserializeResult(de, [], []), deserializeResult(de, deserializeU32(de), deserializeV1(de)), deserializeResult(de, deserializeString(de), deserializeBytes(de))]
                })
            }
        
            /**
* @returns {Promise<Result<number, MyErrno>>} 
*/
            export async function returnResultSugar () {
                const out = []
                

                return fetch('ipc://localhost/variants/return_result_sugar', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeResult(de, deserializeS32(de), deserializeMyErrno(de))
                })
            }
        
            /**
* @returns {Promise<Result<_, MyErrno>>} 
*/
            export async function returnResultSugar2 () {
                const out = []
                

                return fetch('ipc://localhost/variants/return_result_sugar2', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeResult(de, () => {}, deserializeMyErrno(de))
                })
            }
        
            /**
* @returns {Promise<Result<MyErrno, MyErrno>>} 
*/
            export async function returnResultSugar3 () {
                const out = []
                

                return fetch('ipc://localhost/variants/return_result_sugar3', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeResult(de, deserializeMyErrno(de), deserializeMyErrno(de))
                })
            }
        
            /**
* @returns {Promise<Result<[number, number], MyErrno>>} 
*/
            export async function returnResultSugar4 () {
                const out = []
                

                return fetch('ipc://localhost/variants/return_result_sugar4', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeResult(de, [deserializeS32(de), deserializeU32(de)], deserializeMyErrno(de))
                })
            }
        
            /**
* @returns {Promise<number | null>} 
*/
            export async function returnOptionSugar () {
                const out = []
                

                return fetch('ipc://localhost/variants/return_option_sugar', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeOption(de, (de) => deserializeS32(de))
                })
            }
        
            /**
* @returns {Promise<MyErrno | null>} 
*/
            export async function returnOptionSugar2 () {
                const out = []
                

                return fetch('ipc://localhost/variants/return_option_sugar2', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeOption(de, (de) => deserializeMyErrno(de))
                })
            }
        
            /**
* @returns {Promise<Result<number, number>>} 
*/
            export async function resultSimple () {
                const out = []
                

                return fetch('ipc://localhost/variants/result_simple', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeResult(de, deserializeU32(de), deserializeS32(de))
                })
            }
        
            /**
* @param {IsClone} a 
*/
            export async function isCloneArg (a) {
                const out = []
                serializeIsClone(out, a)

                return fetch('ipc://localhost/variants/is_clone_arg', { method: "POST", body: Uint8Array.from(out) })
            }
        
            /**
* @returns {Promise<IsClone>} 
*/
            export async function isCloneReturn () {
                const out = []
                

                return fetch('ipc://localhost/variants/is_clone_return', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeIsClone(de)
                })
            }
        
            /**
* @returns {Promise<[number | null]>} 
*/
            export async function returnNamedOption () {
                const out = []
                

                return fetch('ipc://localhost/variants/return_named_option', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeOption(de, (de) => deserializeU8(de))
                })
            }
        
            /**
* @returns {Promise<[Result<number, MyErrno>]>} 
*/
            export async function returnNamedResult () {
                const out = []
                

                return fetch('ipc://localhost/variants/return_named_result', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeResult(de, deserializeU8(de), deserializeMyErrno(de))
                })
            }
        
