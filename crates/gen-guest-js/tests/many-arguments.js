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
function serializeU64(out, val) {
    return ser_varint(out, 64, val)
}
function serializeString(out, val) {
    serializeU64(out, val.length);

    const encoder = new TextEncoder();

    out.push(...encoder.encode(val))
}
function serializeBigStruct(out, val) {
    serializeString(out, val.a1),
serializeString(out, val.a2),
serializeString(out, val.a3),
serializeString(out, val.a4),
serializeString(out, val.a5),
serializeString(out, val.a6),
serializeString(out, val.a7),
serializeString(out, val.a8),
serializeString(out, val.a9),
serializeString(out, val.a10),
serializeString(out, val.a11),
serializeString(out, val.a12),
serializeString(out, val.a13),
serializeString(out, val.a14),
serializeString(out, val.a15),
serializeString(out, val.a16),
serializeString(out, val.a17),
serializeString(out, val.a18),
serializeString(out, val.a19),
serializeString(out, val.a20)
}

/**
* @param {bigint} a1 
* @param {bigint} a2 
* @param {bigint} a3 
* @param {bigint} a4 
* @param {bigint} a5 
* @param {bigint} a6 
* @param {bigint} a7 
* @param {bigint} a8 
* @param {bigint} a9 
* @param {bigint} a10 
* @param {bigint} a11 
* @param {bigint} a12 
* @param {bigint} a13 
* @param {bigint} a14 
* @param {bigint} a15 
* @param {bigint} a16 
*/
export async function manyArgs (a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16) {
    const out = []
    serializeU64(out, a1);
serializeU64(out, a2);
serializeU64(out, a3);
serializeU64(out, a4);
serializeU64(out, a5);
serializeU64(out, a6);
serializeU64(out, a7);
serializeU64(out, a8);
serializeU64(out, a9);
serializeU64(out, a10);
serializeU64(out, a11);
serializeU64(out, a12);
serializeU64(out, a13);
serializeU64(out, a14);
serializeU64(out, a15);
serializeU64(out, a16)

    return fetch('ipc://localhost/many_arguments/many_args', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
}

/**
* @param {BigStruct} x 
*/
export async function bigArgument (x) {
    const out = []
    serializeBigStruct(out, x)

    return fetch('ipc://localhost/many_arguments/big_argument', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
}

