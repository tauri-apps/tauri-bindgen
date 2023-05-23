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
    let out = 0;

    for (let i = 0; i < varint_max(type); i++) {
        const val = de.pop();
        const carry = val & 0x7F;
        out |= carry << (7 * i);

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
}function ser_varint(out, type, val) {
    let buf = []
    for (let i = 0; i < varint_max(type); i++) {
        const buffer = new ArrayBuffer(type / 8);
        const view = new DataView(buffer);
        view.setInt16(0, val, true);
        buf[i] = view.getUint8(0);
        if (val < 128) {
            out.push(...buf)
            return;
        }

        buf[i] |= 0x80;
        val >>= 7;
    }
    out.push(...buf)
}
function serializeU32(out, val) {
    return ser_varint(out, 32, val)
}

            /**
*/
            export async function f1 () {
                const out = []
                

                return fetch('ipc://localhost/simple_functions/f1', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
            }
        
            /**
* @param {number} a 
*/
            export async function f2 (a) {
                const out = []
                serializeU32(out, a)

                return fetch('ipc://localhost/simple_functions/f2', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
            }
        
            /**
* @param {number} a 
* @param {number} b 
*/
            export async function f3 (a, b) {
                const out = []
                serializeU32(out, a);
serializeU32(out, b)

                return fetch('ipc://localhost/simple_functions/f3', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
            }
        
            /**
* @returns {Promise<number>} 
*/
            export async function f4 () {
                const out = []
                

                return fetch('ipc://localhost/simple_functions/f4', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
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
                const out = []
                

                return fetch('ipc://localhost/simple_functions/f5', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
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
                const out = []
                serializeU32(out, a);
serializeU32(out, b);
serializeU32(out, c)

                return fetch('ipc://localhost/simple_functions/f6', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return [deserializeU32(de), deserializeU32(de), deserializeU32(de)]
                })
            }
        
