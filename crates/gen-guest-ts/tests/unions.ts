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
}function deserializeU16(de) {
    return try_take_varint(de, 16)
}function deserializeU32(de) {
    return try_take_varint(de, 32)
}function deserializeU64(de) {
    return try_take_varint(de, 64)
}function deserializeS8(de) {
    return de.pop()
}function deserializeS16(de) {
    const n = try_take_varint(de, 16)

    return Number(((n >> 1n) & 0xFFFFn) ^ (-((n & 0b1n) & 0xFFFFn)))
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
}function deserializeAllIntegers(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0:
                return deserializeBoolean(de)
            case 1:
                return deserializeU8(de)
            case 2:
                return deserializeU16(de)
            case 3:
                return deserializeU32(de)
            case 4:
                return deserializeU64(de)
            case 5:
                return deserializeS8(de)
            case 6:
                return deserializeS16(de)
            case 7:
                return deserializeS32(de)
            case 8:
                return deserializeS64(de)
            
                    default:
                        throw new Error("unknown union case")
                }
        }function deserializeAllFloats(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0:
                return deserializeF32(de)
            case 1:
                return deserializeF64(de)
            
                    default:
                        throw new Error("unknown union case")
                }
        }function deserializeAllText(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0:
                return deserializeChar(de)
            case 1:
                return deserializeString(de)
            
                    default:
                        throw new Error("unknown union case")
                }
        }function deserializeDuplicatedS32(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0:
                return deserializeS32(de)
            case 1:
                return deserializeS32(de)
            case 2:
                return deserializeS32(de)
            
                    default:
                        throw new Error("unknown union case")
                }
        }function deserializeDistinguishableNum(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0:
                return deserializeF64(de)
            case 1:
                return deserializeS64(de)
            
                    default:
                        throw new Error("unknown union case")
                }
        }
/**
 * A union of all of the integral types 
*/
export type AllIntegers = /**
 * Bool is equivalent to a 1 bit integer 
 * and is treated that way in some languages 
*/
boolean
 | 
number
 | 
number
 | 
number
 | 
bigint
 | 
number
 | 
number
 | 
number
 | 
bigint
;

export type AllFloats = 
number
 | 
number
;

export type AllText = 
string
 | 
string
;

export type DuplicatedS32 = /**
 * The first s32 
*/
number
 | /**
 * The second s32 
*/
number
 | /**
 * The third s32 
*/
number
;
/**
 * A type containing numeric types that are distinct in most languages 
*/
export type DistinguishableNum = /**
 * A Floating Point Number 
*/
number
 | /**
 * A Signed Integer 
*/
bigint
;


            
            export async function addOneInteger (num: AllIntegers) : Promise<AllIntegers> {
                return fetch('ipc://localhost/unions/add_one_integer', { method: "POST", body: JSON.stringify([num]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeAllIntegers(de)
                })
            }
        
            
            export async function addOneFloat (num: AllFloats) : Promise<AllFloats> {
                return fetch('ipc://localhost/unions/add_one_float', { method: "POST", body: JSON.stringify([num]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeAllFloats(de)
                })
            }
        
            
            export async function replaceFirstChar (text: AllText, letter: string) : Promise<AllText> {
                return fetch('ipc://localhost/unions/replace_first_char', { method: "POST", body: JSON.stringify([text, letter]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeAllText(de)
                })
            }
        
            
            export async function identifyInteger (num: AllIntegers) : Promise<number> {
                return fetch('ipc://localhost/unions/identify_integer', { method: "POST", body: JSON.stringify([num]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeU8(de)
                })
            }
        
            
            export async function identifyFloat (num: AllFloats) : Promise<number> {
                return fetch('ipc://localhost/unions/identify_float', { method: "POST", body: JSON.stringify([num]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeU8(de)
                })
            }
        
            
            export async function identifyText (text: AllText) : Promise<number> {
                return fetch('ipc://localhost/unions/identify_text', { method: "POST", body: JSON.stringify([text]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeU8(de)
                })
            }
        
            
            export async function addOneDuplicated (num: DuplicatedS32) : Promise<DuplicatedS32> {
                return fetch('ipc://localhost/unions/add_one_duplicated', { method: "POST", body: JSON.stringify([num]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeDuplicatedS32(de)
                })
            }
        
            
            export async function identifyDuplicated (num: DuplicatedS32) : Promise<number> {
                return fetch('ipc://localhost/unions/identify_duplicated', { method: "POST", body: JSON.stringify([num]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeU8(de)
                })
            }
        
            
            export async function addOneDistinguishableNum (num: DistinguishableNum) : Promise<DistinguishableNum> {
                return fetch('ipc://localhost/unions/add_one_distinguishable_num', { method: "POST", body: JSON.stringify([num]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeDistinguishableNum(de)
                })
            }
        
            
            export async function identifyDistinguishableNum (num: DistinguishableNum) : Promise<number> {
                return fetch('ipc://localhost/unions/identify_distinguishable_num', { method: "POST", body: JSON.stringify([num]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeU8(de)
                })
            }
        