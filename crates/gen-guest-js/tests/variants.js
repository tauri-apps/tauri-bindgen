export class Deserializer {
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
        }
        function deserializeBool(de) {
            const val = de.pop();
        
            return val != 0
        }
        function deserializeU8(de) {
            return de.pop()
        }
        function deserializeU32(de) {
            return try_take_varint(de, 32)
        }
        function deserializeU64(de) {
            return try_take_varint(de, 64)
        }
        function deserializeI32(de) {
            const n = try_take_varint(de, 32)

            return Number(((n >> 1n) as & 0xFFFFFFFFn) ^ (-((n & 0b1n) as & 0xFFFFFFFFn)))
        }
        function deserializeI64(de) {
            const n = try_take_varint(de, u64)

            return Number(((n >> 1n) & 0xFFFFFFFFFFFFFFFFn) ^ (-((n & 0b1n) & 0xFFFFFFFFFFFFFFFFn)))
        }
        function deserializeF32(de) {
            const bytes = de.try_take_n(4);

            const buf = new ArrayBuffer(4);
            const view = new DataView(buf);
        
            bytes.reverse().forEach((v, i) => view.setUint8(i, v));
        
            return view.getFloat32(0);
        }
        function deserializeF64(de) {
            const bytes = de.try_take_n(8);

            const buf = new ArrayBuffer(8);
            const view = new DataView(buf);
        
            bytes.reverse().forEach((v, i) => view.setUint8(i, v));
        
            return view.getFloat64(0);
        }
        function deserializeString(de) {
            const sz = deserializeU64(de);
        
            let bytes = de.try_take_n(Number(sz));
        
            const decoder = new TextDecoder('utf-8');
        
            return decoder.decode(bytes);
        }
        function deserializeBytes(de) {
            const sz = deserializeU64(de);
        
            let bytes = de.try_take_n(Number(sz));
        
            const decoder = new TextDecoder('utf-8');
        
            return decoder.decode(bytes);
        }
        function deserializeOption(de, inner) {
            const disc = de.pop()
        
            switch (disc) {
                case 0:
                    return null
                case 1: 
                    return inner(de)
                default:
                    throw new Error('Deserialize bad option')
            }
        } 
        function function deserializeResult(de, ok, err) {
            const disc = de.pop()
        
            switch (disc) {
                case 0:
                    return ok(de)
                case 1: 
                    return err(de)
                default:
                    throw new Error('Deserialize bad result')
            }
        } 
        function deserializeE1(de) {
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

            /**
* @param {E1} x 
*/
            export async function e1Arg (x) {
                return fetch('ipc://localhost/variants/e1_arg', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @returns {Promise<E1>} 
*/
            export async function e1Result () {
                return fetch('ipc://localhost/variants/e1_result', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeE1(de)
                })
            }
        
            /**
* @param {U1} x 
*/
            export async function u1Arg (x) {
                return fetch('ipc://localhost/variants/u1_arg', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @returns {Promise<U1>} 
*/
            export async function u1Result () {
                return fetch('ipc://localhost/variants/u1_result', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeU1(de)
                })
            }
        
            /**
* @param {V1} x 
*/
            export async function v1Arg (x) {
                return fetch('ipc://localhost/variants/v1_arg', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @returns {Promise<V1>} 
*/
            export async function v1Result () {
                return fetch('ipc://localhost/variants/v1_result', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeV1(de)
                })
            }
        
            /**
* @param {boolean} x 
*/
            export async function boolArg (x) {
                return fetch('ipc://localhost/variants/bool_arg', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @returns {Promise<boolean>} 
*/
            export async function boolResult () {
                return fetch('ipc://localhost/variants/bool_result', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeBoolean(de)
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
                return fetch('ipc://localhost/variants/option_arg', { method: "POST", body: JSON.stringify([a, b, c, d, e, f, g]) })
            }
        
            /**
* @returns {Promise<[boolean | null, [] | null, number | null, E1 | null, number | null, U1 | null, boolean | null | null]>} 
*/
            export async function optionResult () {
                return fetch('ipc://localhost/variants/option_result', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return [deserializeOption(de, (de) => deserializeBoolean(de)), deserializeOption(de, (de) => []), deserializeOption(de, (de) => deserializeU32(de)), deserializeOption(de, (de) => deserializeE1(de)), deserializeOption(de, (de) => deserializeF32(de)), deserializeOption(de, (de) => deserializeU1(de)), deserializeOption(de, (de) => deserializeOption(de, (de) => deserializeBoolean(de)))]
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
                return fetch('ipc://localhost/variants/casts', { method: "POST", body: JSON.stringify([a, b, c, d, e, f]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

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
                return fetch('ipc://localhost/variants/result_arg', { method: "POST", body: JSON.stringify([a, b, c, d, e, f]) })
            }
        
            /**
* @returns {Promise<[Result<_, _>, Result<_, E1>, Result<E1, _>, Result<[], []>, Result<number, V1>, Result<string, Uint8Array[]>]>} 
*/
            export async function resultResult () {
                return fetch('ipc://localhost/variants/result_result', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return [deserializeResult(de, () => {}, () => {}), deserializeResult(de, () => {}, deserializeE1(de)), deserializeResult(de, deserializeE1(de), () => {}), deserializeResult(de, [], []), deserializeResult(de, deserializeU32(de), deserializeV1(de)), deserializeResult(de, deserializeString(de), deserializeBytes(de))]
                })
            }
        
            /**
* @returns {Promise<Result<number, MyErrno>>} 
*/
            export async function returnResultSugar () {
                return fetch('ipc://localhost/variants/return_result_sugar', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeResult(de, deserializeS32(de), deserializeMyErrno(de))
                })
            }
        
            /**
* @returns {Promise<Result<_, MyErrno>>} 
*/
            export async function returnResultSugar2 () {
                return fetch('ipc://localhost/variants/return_result_sugar2', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeResult(de, () => {}, deserializeMyErrno(de))
                })
            }
        
            /**
* @returns {Promise<Result<MyErrno, MyErrno>>} 
*/
            export async function returnResultSugar3 () {
                return fetch('ipc://localhost/variants/return_result_sugar3', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeResult(de, deserializeMyErrno(de), deserializeMyErrno(de))
                })
            }
        
            /**
* @returns {Promise<Result<[number, number], MyErrno>>} 
*/
            export async function returnResultSugar4 () {
                return fetch('ipc://localhost/variants/return_result_sugar4', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeResult(de, [deserializeS32(de), deserializeU32(de)], deserializeMyErrno(de))
                })
            }
        
            /**
* @returns {Promise<number | null>} 
*/
            export async function returnOptionSugar () {
                return fetch('ipc://localhost/variants/return_option_sugar', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeOption(de, (de) => deserializeS32(de))
                })
            }
        
            /**
* @returns {Promise<MyErrno | null>} 
*/
            export async function returnOptionSugar2 () {
                return fetch('ipc://localhost/variants/return_option_sugar2', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeOption(de, (de) => deserializeMyErrno(de))
                })
            }
        
            /**
* @returns {Promise<Result<number, number>>} 
*/
            export async function resultSimple () {
                return fetch('ipc://localhost/variants/result_simple', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeResult(de, deserializeU32(de), deserializeS32(de))
                })
            }
        
            /**
* @param {IsClone} a 
*/
            export async function isCloneArg (a) {
                return fetch('ipc://localhost/variants/is_clone_arg', { method: "POST", body: JSON.stringify([a]) })
            }
        
            /**
* @returns {Promise<IsClone>} 
*/
            export async function isCloneReturn () {
                return fetch('ipc://localhost/variants/is_clone_return', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeIsClone(de)
                })
            }
        
            /**
* @returns {Promise<[number | null]>} 
*/
            export async function returnNamedOption () {
                return fetch('ipc://localhost/variants/return_named_option', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeOption(de, (de) => deserializeU8(de))
                })
            }
        
            /**
* @returns {Promise<[Result<number, MyErrno>]>} 
*/
            export async function returnNamedResult () {
                return fetch('ipc://localhost/variants/return_named_result', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeResult(de, deserializeU8(de), deserializeMyErrno(de))
                })
            }
        
