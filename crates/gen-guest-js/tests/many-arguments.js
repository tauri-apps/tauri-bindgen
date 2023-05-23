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
}function deserializeU64(de) {
    return try_take_varint(de, 64)
}function deserializeString(de) {
    const sz = deserializeU64(de);

    let bytes = de.try_take_n(Number(sz));

    const decoder = new TextDecoder('utf-8');

    return decoder.decode(bytes);
}

            /**
* @param {bigint} a1 
* @param {bigint} a2 
* @param {bigint} a3 
* @param {bigint} a4 
* @param {bigint} a5 
* @param {bigint} a6 
* @param {bigint} a7 
* @param {bigint} a8 
* @param {bigint} a9 
* @param {bigint} a10 
* @param {bigint} a11 
* @param {bigint} a12 
* @param {bigint} a13 
* @param {bigint} a14 
* @param {bigint} a15 
* @param {bigint} a16 
*/
            export async function manyArgs (a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16) {
                return fetch('ipc://localhost/many_arguments/many_args', { method: "POST", body: JSON.stringify([a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16]) })
            }
        
            /**
* @param {BigStruct} x 
*/
            export async function bigArgument (x) {
                return fetch('ipc://localhost/many_arguments/big_argument', { method: "POST", body: JSON.stringify([x]) })
            }
        
