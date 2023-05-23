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
    for (let i = 0; i < varint_max(type); i++) {
        const buffer = new Uint8Array(type / 8);
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
function serializeU32(out, val) {
    return ser_varint(out, 32, val)
}function serializeU64(out, val) {
    return ser_varint(out, 64, val)
}function serializeLudicrousSpeed(out, val) {
                serializeU32(out, val.how_fast_are_you_going),
serializeU64(out, val.i_am_going_extremely_slow)
            }

            /**
*/
            export async function kebabCase () {
                const out = []
                

                return fetch('ipc://localhost/conventions/kebab_case', { method: "POST", body: Uint8Array.from(out) })
            }
        
            /**
* @param {LudicrousSpeed} x 
*/
            export async function foo (x) {
                const out = []
                serializeLudicrousSpeed(out, x)

                return fetch('ipc://localhost/conventions/foo', { method: "POST", body: Uint8Array.from(out) })
            }
        
            /**
*/
            export async function functionWithUnderscores () {
                const out = []
                

                return fetch('ipc://localhost/conventions/function_with_underscores', { method: "POST", body: Uint8Array.from(out) })
            }
        
            /**
*/
            export async function functionWithNoWeirdCharacters () {
                const out = []
                

                return fetch('ipc://localhost/conventions/function_with_no_weird_characters', { method: "POST", body: Uint8Array.from(out) })
            }
        
            /**
*/
            export async function apple () {
                const out = []
                

                return fetch('ipc://localhost/conventions/apple', { method: "POST", body: Uint8Array.from(out) })
            }
        
            /**
*/
            export async function applePear () {
                const out = []
                

                return fetch('ipc://localhost/conventions/apple_pear', { method: "POST", body: Uint8Array.from(out) })
            }
        
            /**
*/
            export async function applePearGrape () {
                const out = []
                

                return fetch('ipc://localhost/conventions/apple_pear_grape', { method: "POST", body: Uint8Array.from(out) })
            }
        
            /**
*/
            export async function a0 () {
                const out = []
                

                return fetch('ipc://localhost/conventions/a0', { method: "POST", body: Uint8Array.from(out) })
            }
        
            /**
*/
            export async function isXml () {
                const out = []
                

                return fetch('ipc://localhost/conventions/is_xml', { method: "POST", body: Uint8Array.from(out) })
            }
        
            /**
*/
            export async function explicit () {
                const out = []
                

                return fetch('ipc://localhost/conventions/explicit', { method: "POST", body: Uint8Array.from(out) })
            }
        
            /**
*/
            export async function explicitSnake () {
                const out = []
                

                return fetch('ipc://localhost/conventions/explicit_snake', { method: "POST", body: Uint8Array.from(out) })
            }
        
            /**
*/
            export async function bool () {
                const out = []
                

                return fetch('ipc://localhost/conventions/bool', { method: "POST", body: Uint8Array.from(out) })
            }
        
