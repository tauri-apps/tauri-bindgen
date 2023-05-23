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
}function deserializeU32(de) {
    return try_take_varint(de, 32)
}function deserializeU64(de) {
    return try_take_varint(de, 64)
}function deserializeList(de, inner) {
    const len = deserializeU64(de);

    let out = [];

    for (let i = 0; i < len; i++) {
        out.push(inner(de));   
    }

    return out;
}

            /**
* @param {Uint32Array[]} l 
*/
            export async function simpleList1 (l) {
                return fetch('ipc://localhost/simple_lists/simple_list1', { method: "POST", body: JSON.stringify([l]) })
            }
        
            /**
* @returns {Promise<Uint32Array[]>} 
*/
            export async function simpleList2 () {
                return fetch('ipc://localhost/simple_lists/simple_list2', { method: "POST", body: JSON.stringify([]) })
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
                return fetch('ipc://localhost/simple_lists/simple_list3', { method: "POST", body: JSON.stringify([a, b]) })
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
                return fetch('ipc://localhost/simple_lists/simple_list4', { method: "POST", body: JSON.stringify([l]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeList(de, (de) => deserializeList(de, (de) => deserializeU32(de)))
                })
            }
        
