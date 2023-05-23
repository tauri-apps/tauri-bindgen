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
function ser_varint(out, type, val) {
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
}function serializeU64(out, val) {
    return ser_varint(out, 64, val)
}function serializeLudicrousSpeed(out, val) {
                serializeU32(out, val.how_fast_are_you_going),
serializeU64(out, val.i_am_going_extremely_slow)
            }

export interface LudicrousSpeed { 
howFastAreYouGoing: number,

iAmGoingExtremelySlow: bigint,
 }


            
            export async function kebabCase () : Promise<void> {
                const out = []
                
                
                 fetch('ipc://localhost/conventions/kebab_case', { method: "POST", body: Uint8Array.from(out) }) 
            }
        
            
            export async function foo (x: LudicrousSpeed) : Promise<void> {
                const out = []
                serializeLudicrousSpeed(out, x)
                
                 fetch('ipc://localhost/conventions/foo', { method: "POST", body: Uint8Array.from(out) }) 
            }
        
            
            export async function functionWithUnderscores () : Promise<void> {
                const out = []
                
                
                 fetch('ipc://localhost/conventions/function_with_underscores', { method: "POST", body: Uint8Array.from(out) }) 
            }
        
            
            export async function functionWithNoWeirdCharacters () : Promise<void> {
                const out = []
                
                
                 fetch('ipc://localhost/conventions/function_with_no_weird_characters', { method: "POST", body: Uint8Array.from(out) }) 
            }
        
            
            export async function apple () : Promise<void> {
                const out = []
                
                
                 fetch('ipc://localhost/conventions/apple', { method: "POST", body: Uint8Array.from(out) }) 
            }
        
            
            export async function applePear () : Promise<void> {
                const out = []
                
                
                 fetch('ipc://localhost/conventions/apple_pear', { method: "POST", body: Uint8Array.from(out) }) 
            }
        
            
            export async function applePearGrape () : Promise<void> {
                const out = []
                
                
                 fetch('ipc://localhost/conventions/apple_pear_grape', { method: "POST", body: Uint8Array.from(out) }) 
            }
        
            
            export async function a0 () : Promise<void> {
                const out = []
                
                
                 fetch('ipc://localhost/conventions/a0', { method: "POST", body: Uint8Array.from(out) }) 
            }
        
            
            export async function isXml () : Promise<void> {
                const out = []
                
                
                 fetch('ipc://localhost/conventions/is_xml', { method: "POST", body: Uint8Array.from(out) }) 
            }
        
            
            export async function explicit () : Promise<void> {
                const out = []
                
                
                 fetch('ipc://localhost/conventions/explicit', { method: "POST", body: Uint8Array.from(out) }) 
            }
        
            
            export async function explicitSnake () : Promise<void> {
                const out = []
                
                
                 fetch('ipc://localhost/conventions/explicit_snake', { method: "POST", body: Uint8Array.from(out) }) 
            }
        
            
            export async function bool () : Promise<void> {
                const out = []
                
                
                 fetch('ipc://localhost/conventions/bool', { method: "POST", body: Uint8Array.from(out) }) 
            }
        