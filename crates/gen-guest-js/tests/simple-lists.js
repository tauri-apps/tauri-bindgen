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
}function deserializeU32(de) {
    return de_varint(de, 32)
}function deserializeU64(de) {
    return de_varint(de, 64)
}function deserializeList(de, inner) {
    const len = deserializeU64(de);

    let out = [];

    for (let i = 0; i < len; i++) {
        out.push(inner(de));   
    }

    return out;
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
function serializeU32(out, val) {
    return ser_varint(out, 32, val)
}function serializeU64(out, val) {
    return ser_varint(out, 64, val)
}function serializeList(out, inner, val) {
    serializeU64(out, val.length)
    for (const el in val) {
        inner(out, el)
    }
}

            /**
* @param {Uint32Array[]} l 
*/
            export async function simpleList1 (l) {
                const out = []
                serializeList(out, (v) => serializeU32(out, v), l)

                return fetch('ipc://localhost/simple_lists/simple_list1', { method: "POST", body: Uint8Array.from(out) })
            }
        
            /**
* @returns {Promise<Uint32Array[]>} 
*/
            export async function simpleList2 () {
                const out = []
                

                return fetch('ipc://localhost/simple_lists/simple_list2', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeList(de, (de) => deserializeU32(de))
                })
            }
        
            /**
* @param {Uint32Array[]} a 
* @param {Uint32Array[]} b 
* @returns {Promise<[Uint32Array[], Uint32Array[]]>} 
*/
            export async function simpleList3 (a, b) {
                const out = []
                serializeList(out, (v) => serializeU32(out, v), a);
serializeList(out, (v) => serializeU32(out, v), b)

                return fetch('ipc://localhost/simple_lists/simple_list3', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return [deserializeList(de, (de) => deserializeU32(de)), deserializeList(de, (de) => deserializeU32(de))]
                })
            }
        
            /**
* @param {Uint32Array[][]} l 
* @returns {Promise<Uint32Array[][]>} 
*/
            export async function simpleList4 (l) {
                const out = []
                serializeList(out, (v) => serializeList(out, (v) => serializeU32(out, v), v), l)

                return fetch('ipc://localhost/simple_lists/simple_list4', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeList(de, (de) => deserializeList(de, (de) => deserializeU32(de)))
                })
            }
        
