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
}function deserializeU16(de) {
    return de_varint(de, 16)
}function deserializeU32(de) {
    return de_varint(de, 32)
}function deserializeU64(de) {
    return de_varint(de, 64)
}function deserializeS8(de) {
    return de.pop()
}function deserializeS16(de) {
    const n = de_varint(de, 16)

    return Number(((n >> 1n) & 0xFFFFn) ^ (-((n & 0b1n) & 0xFFFFn)))
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
}function deserializeChar(de) {
    const sz = deserializeU64(de);
    if (sz > 4) {
        throw new Error("Deserialize bad char");
    }
    const bytes = de.try_take_n(Number(sz));

    const decoder = new TextDecoder('utf-8');

    return decoder.decode(bytes);
}function deserializeString(de) {
    const sz = deserializeU64(de);

    let bytes = de.try_take_n(Number(sz));

    const decoder = new TextDecoder('utf-8');

    return decoder.decode(bytes);
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
}function serializeU16(out, val) {
    return ser_varint(out, 16, val)
}function serializeU32(out, val) {
    return ser_varint(out, 32, val)
}function serializeU64(out, val) {
    return ser_varint(out, 64, val)
}function serializeS8(out, val) {
    out.push(val)
}function serializeS16(out, val) {
    ser_varint(out, 16, BigInt((val << 1) ^ (val >> 15)))
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
}function serializeChar(out, val) {
    if (val.len > 1) {
        throw new Error("Serialize bad char");
    }

    serializeU64(out, val.length);

    const encoder = new TextEncoder();

    out.push(...encoder.encode(val))
}function serializeString(out, val) {
    serializeU64(out, val.length);

    const encoder = new TextEncoder();

    out.push(...encoder.encode(val))
}function deserializeAllIntegers(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0n:
                return { tag: 0, value: deserializeBool(de) }
            case 1n:
                return { tag: 1, value: deserializeU8(de) }
            case 2n:
                return { tag: 2, value: deserializeU16(de) }
            case 3n:
                return { tag: 3, value: deserializeU32(de) }
            case 4n:
                return { tag: 4, value: deserializeU64(de) }
            case 5n:
                return { tag: 5, value: deserializeS8(de) }
            case 6n:
                return { tag: 6, value: deserializeS16(de) }
            case 7n:
                return { tag: 7, value: deserializeS32(de) }
            case 8n:
                return { tag: 8, value: deserializeS64(de) }
            
                    default:
                        throw new Error("unknown variant case")
                }
        }function deserializeAllFloats(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0n:
                return { tag: 0, value: deserializeF32(de) }
            case 1n:
                return { tag: 1, value: deserializeF64(de) }
            
                    default:
                        throw new Error("unknown variant case")
                }
        }function deserializeAllText(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0n:
                return { tag: 0, value: deserializeChar(de) }
            case 1n:
                return { tag: 1, value: deserializeString(de) }
            
                    default:
                        throw new Error("unknown variant case")
                }
        }function deserializeDuplicatedS32(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0n:
                return { tag: 0, value: deserializeS32(de) }
            case 1n:
                return { tag: 1, value: deserializeS32(de) }
            case 2n:
                return { tag: 2, value: deserializeS32(de) }
            
                    default:
                        throw new Error("unknown variant case")
                }
        }function deserializeDistinguishableNum(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0n:
                return { tag: 0, value: deserializeF64(de) }
            case 1n:
                return { tag: 1, value: deserializeS64(de) }
            
                    default:
                        throw new Error("unknown variant case")
                }
        }function serializeAllIntegers(out, val) {
                serializeU32(out, val.tag)

                switch (val.tag) {
                    case 0:
                    serializeBool(out, val.val)
                    return
                case 1:
                    serializeU8(out, val.val)
                    return
                case 2:
                    serializeU16(out, val.val)
                    return
                case 3:
                    serializeU32(out, val.val)
                    return
                case 4:
                    serializeU64(out, val.val)
                    return
                case 5:
                    serializeS8(out, val.val)
                    return
                case 6:
                    serializeS16(out, val.val)
                    return
                case 7:
                    serializeS32(out, val.val)
                    return
                case 8:
                    serializeS64(out, val.val)
                    return
                
                    default:
                        throw new Error("unknown union case")
                }
        }function serializeAllFloats(out, val) {
                serializeU32(out, val.tag)

                switch (val.tag) {
                    case 0:
                    serializeF32(out, val.val)
                    return
                case 1:
                    serializeF64(out, val.val)
                    return
                
                    default:
                        throw new Error("unknown union case")
                }
        }function serializeAllText(out, val) {
                serializeU32(out, val.tag)

                switch (val.tag) {
                    case 0:
                    serializeChar(out, val.val)
                    return
                case 1:
                    serializeString(out, val.val)
                    return
                
                    default:
                        throw new Error("unknown union case")
                }
        }function serializeDuplicatedS32(out, val) {
                serializeU32(out, val.tag)

                switch (val.tag) {
                    case 0:
                    serializeS32(out, val.val)
                    return
                case 1:
                    serializeS32(out, val.val)
                    return
                case 2:
                    serializeS32(out, val.val)
                    return
                
                    default:
                        throw new Error("unknown union case")
                }
        }function serializeDistinguishableNum(out, val) {
                serializeU32(out, val.tag)

                switch (val.tag) {
                    case 0:
                    serializeF64(out, val.val)
                    return
                case 1:
                    serializeS64(out, val.val)
                    return
                
                    default:
                        throw new Error("unknown union case")
                }
        }

            /**
* @param {AllIntegers} num 
* @returns {Promise<AllIntegers>} 
*/
            export async function addOneInteger (num) {
                const out = []
                serializeAllIntegers(out, num)

                return fetch('ipc://localhost/unions/add_one_integer', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeAllIntegers(de)
                })
            }
        
            /**
* @param {AllFloats} num 
* @returns {Promise<AllFloats>} 
*/
            export async function addOneFloat (num) {
                const out = []
                serializeAllFloats(out, num)

                return fetch('ipc://localhost/unions/add_one_float', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeAllFloats(de)
                })
            }
        
            /**
* @param {AllText} text 
* @param {string} letter 
* @returns {Promise<AllText>} 
*/
            export async function replaceFirstChar (text, letter) {
                const out = []
                serializeAllText(out, text);
serializeChar(out, letter)

                return fetch('ipc://localhost/unions/replace_first_char', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeAllText(de)
                })
            }
        
            /**
* @param {AllIntegers} num 
* @returns {Promise<number>} 
*/
            export async function identifyInteger (num) {
                const out = []
                serializeAllIntegers(out, num)

                return fetch('ipc://localhost/unions/identify_integer', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeU8(de)
                })
            }
        
            /**
* @param {AllFloats} num 
* @returns {Promise<number>} 
*/
            export async function identifyFloat (num) {
                const out = []
                serializeAllFloats(out, num)

                return fetch('ipc://localhost/unions/identify_float', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeU8(de)
                })
            }
        
            /**
* @param {AllText} text 
* @returns {Promise<number>} 
*/
            export async function identifyText (text) {
                const out = []
                serializeAllText(out, text)

                return fetch('ipc://localhost/unions/identify_text', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeU8(de)
                })
            }
        
            /**
* @param {DuplicatedS32} num 
* @returns {Promise<DuplicatedS32>} 
*/
            export async function addOneDuplicated (num) {
                const out = []
                serializeDuplicatedS32(out, num)

                return fetch('ipc://localhost/unions/add_one_duplicated', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeDuplicatedS32(de)
                })
            }
        
            /**
* @param {DuplicatedS32} num 
* @returns {Promise<number>} 
*/
            export async function identifyDuplicated (num) {
                const out = []
                serializeDuplicatedS32(out, num)

                return fetch('ipc://localhost/unions/identify_duplicated', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeU8(de)
                })
            }
        
            /**
* @param {DistinguishableNum} num 
* @returns {Promise<DistinguishableNum>} 
*/
            export async function addOneDistinguishableNum (num) {
                const out = []
                serializeDistinguishableNum(out, num)

                return fetch('ipc://localhost/unions/add_one_distinguishable_num', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeDistinguishableNum(de)
                })
            }
        
            /**
* @param {DistinguishableNum} num 
* @returns {Promise<number>} 
*/
            export async function identifyDistinguishableNum (num) {
                const out = []
                serializeDistinguishableNum(out, num)

                return fetch('ipc://localhost/unions/identify_distinguishable_num', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeU8(de)
                })
            }
        
