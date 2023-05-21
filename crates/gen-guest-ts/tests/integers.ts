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


            
            export async function a1 (x: number)  {
                return fetch('ipc://localhost/integers/a1', { method: "POST", body: JSON.stringify([x]) })
            }
        
            
            export async function a2 (x: number)  {
                return fetch('ipc://localhost/integers/a2', { method: "POST", body: JSON.stringify([x]) })
            }
        
            
            export async function a3 (x: number)  {
                return fetch('ipc://localhost/integers/a3', { method: "POST", body: JSON.stringify([x]) })
            }
        
            
            export async function a4 (x: number)  {
                return fetch('ipc://localhost/integers/a4', { method: "POST", body: JSON.stringify([x]) })
            }
        
            
            export async function a5 (x: number)  {
                return fetch('ipc://localhost/integers/a5', { method: "POST", body: JSON.stringify([x]) })
            }
        
            
            export async function a6 (x: number)  {
                return fetch('ipc://localhost/integers/a6', { method: "POST", body: JSON.stringify([x]) })
            }
        
            
            export async function a7 (x: bigint)  {
                return fetch('ipc://localhost/integers/a7', { method: "POST", body: JSON.stringify([x]) })
            }
        
            
            export async function a8 (x: bigint)  {
                return fetch('ipc://localhost/integers/a8', { method: "POST", body: JSON.stringify([x]) })
            }
        
            
            export async function a9 (p1: number, p2: number, p3: number, p4: number, p5: number, p6: number, p7: bigint, p8: bigint)  {
                return fetch('ipc://localhost/integers/a9', { method: "POST", body: JSON.stringify([p1, p2, p3, p4, p5, p6, p7, p8]) })
            }
        
            
            export async function r1 () : Promise<number> {
                return fetch('ipc://localhost/integers/r1', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeU8(de)
                })
            }
        
            
            export async function r2 () : Promise<number> {
                return fetch('ipc://localhost/integers/r2', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeS8(de)
                })
            }
        
            
            export async function r3 () : Promise<number> {
                return fetch('ipc://localhost/integers/r3', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeU16(de)
                })
            }
        
            
            export async function r4 () : Promise<number> {
                return fetch('ipc://localhost/integers/r4', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeS16(de)
                })
            }
        
            
            export async function r5 () : Promise<number> {
                return fetch('ipc://localhost/integers/r5', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeU32(de)
                })
            }
        
            
            export async function r6 () : Promise<number> {
                return fetch('ipc://localhost/integers/r6', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeS32(de)
                })
            }
        
            
            export async function r7 () : Promise<bigint> {
                return fetch('ipc://localhost/integers/r7', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeU64(de)
                })
            }
        
            
            export async function r8 () : Promise<bigint> {
                return fetch('ipc://localhost/integers/r8', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeS64(de)
                })
            }
        
            
            export async function pairRet () : Promise<[bigint, number]> {
                return fetch('ipc://localhost/integers/pair_ret', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return [deserializeS64(de), deserializeU8(de)]
                })
            }
        