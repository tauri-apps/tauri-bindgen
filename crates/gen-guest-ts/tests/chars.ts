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
}function deserializeU64(de) {
    return de_varint(de, 64)
}function deserializeChar(de) {
    const sz = deserializeU64(de);
    if (sz > 4) {
        throw new Error("Deserialize bad char");
    }
    const bytes = de.try_take_n(Number(sz));

    const decoder = new TextDecoder('utf-8');

    return decoder.decode(bytes);
}function ser_varint(out, type, val) {
    for (let i = 0; i < varint_max(type); i++) {
        const buffer = new ArrayBuffer(type / 8);
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
function serializeU64(out, val) {
    return ser_varint(out, 64, val)
}function serializeChar(out, val) {
    if (val.len > 1) {
        throw new Error("Serialize bad char");
    }

    serializeU64(out, val.length);

    const encoder = new TextEncoder();

    out.push(...encoder.encode(val))
}


            /**
 * A function that accepts a character 
*/
            export async function takeChar (x: string) : Promise<void> {
                const out = []
                serializeChar(out, x)
                
                 fetch('ipc://localhost/chars/take_char', { method: "POST", body: Uint8Array.from(out) }) 
            }
        
            /**
 * A function that returns a character 
*/
            export async function returnChar () : Promise<string> {
                const out = []
                
                
                return fetch('ipc://localhost/chars/return_char', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeChar(de)
                }) as Promise<string>
            }
        