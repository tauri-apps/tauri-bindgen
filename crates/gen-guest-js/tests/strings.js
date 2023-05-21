export class Deserializer {
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
        function deserializeString(de) {
            const sz = deserializeU64(de);
        
            let bytes = de.try_take_n(Number(sz));
        
            const decoder = new TextDecoder('utf-8');
        
            return decoder.decode(bytes);
        }
        

            /**
* @param {string} x 
*/
            export async function a (x) {
                return fetch('ipc://localhost/strings/a', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @returns {Promise<string>} 
*/
            export async function b () {
                return fetch('ipc://localhost/strings/b', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeString(de)
                })
            }
        
            /**
* @param {string} a 
* @param {string} b 
* @returns {Promise<string>} 
*/
            export async function c (a, b) {
                return fetch('ipc://localhost/strings/c', { method: "POST", body: JSON.stringify([a, b]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeString(de)
                })
            }
        
