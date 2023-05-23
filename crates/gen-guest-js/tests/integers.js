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
}

            /**
* @param {number} x 
*/
            export async function a1 (x) {
                return fetch('ipc://localhost/integers/a1', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @param {number} x 
*/
            export async function a2 (x) {
                return fetch('ipc://localhost/integers/a2', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @param {number} x 
*/
            export async function a3 (x) {
                return fetch('ipc://localhost/integers/a3', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @param {number} x 
*/
            export async function a4 (x) {
                return fetch('ipc://localhost/integers/a4', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @param {number} x 
*/
            export async function a5 (x) {
                return fetch('ipc://localhost/integers/a5', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @param {number} x 
*/
            export async function a6 (x) {
                return fetch('ipc://localhost/integers/a6', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @param {bigint} x 
*/
            export async function a7 (x) {
                return fetch('ipc://localhost/integers/a7', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @param {bigint} x 
*/
            export async function a8 (x) {
                return fetch('ipc://localhost/integers/a8', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @param {number} p1 
* @param {number} p2 
* @param {number} p3 
* @param {number} p4 
* @param {number} p5 
* @param {number} p6 
* @param {bigint} p7 
* @param {bigint} p8 
*/
            export async function a9 (p1, p2, p3, p4, p5, p6, p7, p8) {
                return fetch('ipc://localhost/integers/a9', { method: "POST", body: JSON.stringify([p1, p2, p3, p4, p5, p6, p7, p8]) })
            }
        
            /**
* @returns {Promise<number>} 
*/
            export async function r1 () {
                return fetch('ipc://localhost/integers/r1', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeU8(de)
                })
            }
        
            /**
* @returns {Promise<number>} 
*/
            export async function r2 () {
                return fetch('ipc://localhost/integers/r2', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeS8(de)
                })
            }
        
            /**
* @returns {Promise<number>} 
*/
            export async function r3 () {
                return fetch('ipc://localhost/integers/r3', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeU16(de)
                })
            }
        
            /**
* @returns {Promise<number>} 
*/
            export async function r4 () {
                return fetch('ipc://localhost/integers/r4', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeS16(de)
                })
            }
        
            /**
* @returns {Promise<number>} 
*/
            export async function r5 () {
                return fetch('ipc://localhost/integers/r5', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeU32(de)
                })
            }
        
            /**
* @returns {Promise<number>} 
*/
            export async function r6 () {
                return fetch('ipc://localhost/integers/r6', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeS32(de)
                })
            }
        
            /**
* @returns {Promise<bigint>} 
*/
            export async function r7 () {
                return fetch('ipc://localhost/integers/r7', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeU64(de)
                })
            }
        
            /**
* @returns {Promise<bigint>} 
*/
            export async function r8 () {
                return fetch('ipc://localhost/integers/r8', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeS64(de)
                })
            }
        
            /**
* @returns {Promise<[bigint, number]>} 
*/
            export async function pairRet () {
                return fetch('ipc://localhost/integers/pair_ret', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return [deserializeS64(de), deserializeU8(de)]
                })
            }
        
