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
}function deserializeU8(de) {
    return de.pop()
}function deserializeU16(de) {
    return de_varint(de, 16)
}function deserializeU32(de) {
    return de_varint(de, 32)
}function deserializeU64(de) {
    return de_varint(de, 64)
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
function serializeU8(out, val) {
    return out.push(val)
}function serializeU16(out, val) {
    return ser_varint(out, 16, val)
}function serializeU32(out, val) {
    return ser_varint(out, 32, val)
}function serializeU64(out, val) {
    return ser_varint(out, 64, val)
}function deserializeFlag1(de) {
                return deserializeU8(de)
        }function deserializeFlag2(de) {
                return deserializeU8(de)
        }function deserializeFlag4(de) {
                return deserializeU8(de)
        }function deserializeFlag8(de) {
                return deserializeU8(de)
        }function deserializeFlag16(de) {
                return deserializeU16(de)
        }function deserializeFlag32(de) {
                return deserializeU32(de)
        }function deserializeFlag64(de) {
                return deserializeU64(de)
        }function serializeFlag1(out, val) {
                return serializeU8(out, val)
        }function serializeFlag2(out, val) {
                return serializeU8(out, val)
        }function serializeFlag4(out, val) {
                return serializeU8(out, val)
        }function serializeFlag8(out, val) {
                return serializeU8(out, val)
        }function serializeFlag16(out, val) {
                return serializeU16(out, val)
        }function serializeFlag32(out, val) {
                return serializeU32(out, val)
        }function serializeFlag64(out, val) {
                return serializeU64(out, val)
        }

            /**
* @param {Flag1} x 
* @returns {Promise<Flag1>} 
*/
            export async function roundtripFlag1 (x) {
                const out = []
                serializeFlag1(out, x)

                return fetch('ipc://localhost/flegs/roundtrip_flag1', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeFlag1(de)
                })
            }
        
            /**
* @param {Flag2} x 
* @returns {Promise<Flag2>} 
*/
            export async function roundtripFlag2 (x) {
                const out = []
                serializeFlag2(out, x)

                return fetch('ipc://localhost/flegs/roundtrip_flag2', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeFlag2(de)
                })
            }
        
            /**
* @param {Flag4} x 
* @returns {Promise<Flag4>} 
*/
            export async function roundtripFlag4 (x) {
                const out = []
                serializeFlag4(out, x)

                return fetch('ipc://localhost/flegs/roundtrip_flag4', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeFlag4(de)
                })
            }
        
            /**
* @param {Flag8} x 
* @returns {Promise<Flag8>} 
*/
            export async function roundtripFlag8 (x) {
                const out = []
                serializeFlag8(out, x)

                return fetch('ipc://localhost/flegs/roundtrip_flag8', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeFlag8(de)
                })
            }
        
            /**
* @param {Flag16} x 
* @returns {Promise<Flag16>} 
*/
            export async function roundtripFlag16 (x) {
                const out = []
                serializeFlag16(out, x)

                return fetch('ipc://localhost/flegs/roundtrip_flag16', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeFlag16(de)
                })
            }
        
            /**
* @param {Flag32} x 
* @returns {Promise<Flag32>} 
*/
            export async function roundtripFlag32 (x) {
                const out = []
                serializeFlag32(out, x)

                return fetch('ipc://localhost/flegs/roundtrip_flag32', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeFlag32(de)
                })
            }
        
            /**
* @param {Flag64} x 
* @returns {Promise<Flag64>} 
*/
            export async function roundtripFlag64 (x) {
                const out = []
                serializeFlag64(out, x)

                return fetch('ipc://localhost/flegs/roundtrip_flag64', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeFlag64(de)
                })
            }
        
