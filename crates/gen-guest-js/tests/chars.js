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
        }
        function deserializeU64(de) {
            return try_take_varint(de, 64)
        }
        function deserializeChar(de) {
            const sz = deserializeU64(de);
            if (sz > 4) {
                throw new Error("Deserialize bad char");
            }
            const bytes = de.try_take_n(Number(sz));
        
            const decoder = new TextDecoder('utf-8');
        
            return decoder.decode(bytes);
        }
        

            /**
 * A function that accepts a character 
* @param {string} x 
*/
            export async function takeChar (x) {
                return fetch('ipc://localhost/chars/take_char', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
 * A function that returns a character 
* @returns {Promise<string>} 
*/
            export async function returnChar () {
                return fetch('ipc://localhost/chars/return_char', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeChar(de)
                })
            }
        
