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
}

            /**
*/
            export async function f1 () {
                return fetch('ipc://localhost/simple_functions/f1', { method: "POST", body: JSON.stringify([]) })
            }
        
            /**
* @param {number} a 
*/
            export async function f2 (a) {
                return fetch('ipc://localhost/simple_functions/f2', { method: "POST", body: JSON.stringify([a]) })
            }
        
            /**
* @param {number} a 
* @param {number} b 
*/
            export async function f3 (a, b) {
                return fetch('ipc://localhost/simple_functions/f3', { method: "POST", body: JSON.stringify([a, b]) })
            }
        
            /**
* @returns {Promise<number>} 
*/
            export async function f4 () {
                return fetch('ipc://localhost/simple_functions/f4', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeU32(de)
                })
            }
        
            /**
* @returns {Promise<[number, number]>} 
*/
            export async function f5 () {
                return fetch('ipc://localhost/simple_functions/f5', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return [deserializeU32(de), deserializeU32(de)]
                })
            }
        
            /**
* @param {number} a 
* @param {number} b 
* @param {number} c 
* @returns {Promise<[number, number, number]>} 
*/
            export async function f6 (a, b, c) {
                return fetch('ipc://localhost/simple_functions/f6', { method: "POST", body: JSON.stringify([a, b, c]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return [deserializeU32(de), deserializeU32(de), deserializeU32(de)]
                })
            }
        
