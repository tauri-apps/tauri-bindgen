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
}function deserializeString(de) {
    const sz = deserializeU64(de);

    let bytes = de.try_take_n(Number(sz));

    const decoder = new TextDecoder('utf-8');

    return decoder.decode(bytes);
}function deserializeOption(de, inner) {
    const disc = de.pop()

    switch (disc) {
        case 0:
            return null
        case 1: 
            return inner(de)
        default:
            throw new Error('Deserialize bad option')
    }
}function deserializeResult(de, ok, err) {
    const disc = de.pop()

    switch (disc) {
        case 0:
            return ok(de)
        case 1: 
            return err(de)
        default:
            throw new Error('Deserialize bad result')
    }
}function deserializeError(de) {
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0n:
                return "Success"
            case 1n:
                return "Failure"
            
                    default:
                        throw new Error("unknown enum case")
                }
        }

            /**
* @returns {Promise<Result<string | null, Error>>} 
*/
            export async function optionTest () {
                const out = []
                

                return fetch('ipc://localhost/small_anonymous/option_test', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeResult(de, deserializeOption(de, (de) => deserializeString(de)), deserializeError(de))
                })
            }
        
