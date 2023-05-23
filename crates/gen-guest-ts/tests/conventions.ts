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
}function deserializeU64(de) {
    return try_take_varint(de, 64)
}

export interface LudicrousSpeed { 
howFastAreYouGoing: number,

iAmGoingExtremelySlow: bigint,
 }


            
            export async function kebabCase ()  {
                return fetch('ipc://localhost/conventions/kebab_case', { method: "POST", body: JSON.stringify([]) })
            }
        
            
            export async function foo (x: LudicrousSpeed)  {
                return fetch('ipc://localhost/conventions/foo', { method: "POST", body: JSON.stringify([x]) })
            }
        
            
            export async function functionWithUnderscores ()  {
                return fetch('ipc://localhost/conventions/function_with_underscores', { method: "POST", body: JSON.stringify([]) })
            }
        
            
            export async function functionWithNoWeirdCharacters ()  {
                return fetch('ipc://localhost/conventions/function_with_no_weird_characters', { method: "POST", body: JSON.stringify([]) })
            }
        
            
            export async function apple ()  {
                return fetch('ipc://localhost/conventions/apple', { method: "POST", body: JSON.stringify([]) })
            }
        
            
            export async function applePear ()  {
                return fetch('ipc://localhost/conventions/apple_pear', { method: "POST", body: JSON.stringify([]) })
            }
        
            
            export async function applePearGrape ()  {
                return fetch('ipc://localhost/conventions/apple_pear_grape', { method: "POST", body: JSON.stringify([]) })
            }
        
            
            export async function a0 ()  {
                return fetch('ipc://localhost/conventions/a0', { method: "POST", body: JSON.stringify([]) })
            }
        
            
            export async function isXml ()  {
                return fetch('ipc://localhost/conventions/is_xml', { method: "POST", body: JSON.stringify([]) })
            }
        
            
            export async function explicit ()  {
                return fetch('ipc://localhost/conventions/explicit', { method: "POST", body: JSON.stringify([]) })
            }
        
            
            export async function explicitSnake ()  {
                return fetch('ipc://localhost/conventions/explicit_snake', { method: "POST", body: JSON.stringify([]) })
            }
        
            
            export async function bool ()  {
                return fetch('ipc://localhost/conventions/bool', { method: "POST", body: JSON.stringify([]) })
            }
        