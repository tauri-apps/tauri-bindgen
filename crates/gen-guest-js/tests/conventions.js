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
// function varint_max(bits) {
//   const BITS_PER_BYTE = 8;
//   const BITS_PER_VARINT_BYTE = 7;

//   const roundup_bits = bits + (BITS_PER_BYTE - 1);

//   return Math.floor(roundup_bits / BITS_PER_VARINT_BYTE);
// }

const varint_max = {
  16: 3,
  32: 5,
  64: 10,
  128: 19
}
function ser_varint(out, bits, val) {
  let buf = []
  for (let i = 0; i < varint_max[bits]; i++) {
    const buffer = new ArrayBuffer(bits / 8);
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

function ser_varint_big(out, bits, val) {
  let buf = []
  for (let i = 0; i < varint_max[bits]; i++) {
    const buffer = new ArrayBuffer(bits / 8);
    const view = new DataView(buffer);
    view.setInt16(0, Number(val), true);
    buf[i] = view.getUint8(0);
    if (val < 128) {
      out.push(...buf)
      return;
    }

    buf[i] |= 0x80;
    val >>= 7n;
  }
  out.push(...buf)
}
function serializeU32(out, val) {
    return ser_varint(out, 32, val)
}
function serializeU64(out, val) {
  return ser_varint_big(out, 64, BigInt(val))
}
function serializeLudicrousSpeed(out, val) {
    serializeU32(out, val.how_fast_are_you_going),
serializeU64(out, val.i_am_going_extremely_slow)
}

/**
*/
export async function kebabCase () {
    const out = []
    

    return fetch('ipc://localhost/conventions/kebab_case', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
}

/**
* @param {LudicrousSpeed} x
*/
export async function foo (x) {
    const out = []
    serializeLudicrousSpeed(out, x)

    return fetch('ipc://localhost/conventions/foo', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
}

/**
*/
export async function functionWithUnderscores () {
    const out = []
    

    return fetch('ipc://localhost/conventions/function_with_underscores', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
}

/**
*/
export async function functionWithNoWeirdCharacters () {
    const out = []
    

    return fetch('ipc://localhost/conventions/function_with_no_weird_characters', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
}

/**
*/
export async function apple () {
    const out = []
    

    return fetch('ipc://localhost/conventions/apple', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
}

/**
*/
export async function applePear () {
    const out = []
    

    return fetch('ipc://localhost/conventions/apple_pear', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
}

/**
*/
export async function applePearGrape () {
    const out = []
    

    return fetch('ipc://localhost/conventions/apple_pear_grape', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
}

/**
*/
export async function a0 () {
    const out = []
    

    return fetch('ipc://localhost/conventions/a0', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
}

/**
*/
export async function isXml () {
    const out = []
    

    return fetch('ipc://localhost/conventions/is_xml', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
}

/**
*/
export async function explicit () {
    const out = []
    

    return fetch('ipc://localhost/conventions/explicit', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
}

/**
*/
export async function explicitSnake () {
    const out = []
    

    return fetch('ipc://localhost/conventions/explicit_snake', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
}

/**
*/
export async function bool () {
    const out = []
    

    return fetch('ipc://localhost/conventions/bool', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
}

