const varintMax = {
    16: 3,
    32: 5,
    64: 10
}
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
function deVarint(de, bits) {
    let out = 0;

    for (let i = 0; i < varintMax[bits]; i++) {
        const val = de.pop();
        const carry = val & 0x7F;
        out |= carry << (7 * i);

        if ((val & 0x80) === 0) {
            if (i === varintMax[bits] - 1 && val > ((1 << bits % 7) - 1)) {
                throw new Error('deserialize bad variant')
            } else {
                return out
            }
        }
    }

    throw new Error('deserialize bad variant')
}function deBool(de) {
    const val = de.pop();

    return val != 0
}
function deU8(de) {
    return de.pop()
}
function deU16(de) {
    return deVarint(de, 16)
}
function deU32(de) {
    return deVarint(de, 32)
}
function deU64(de) {
    return deVarint(de, 64)
}
function deS8(de) {
    const view = new DataView(de.source.buffer, de.offset, 1);
    de.offset += 1;
    return view.getInt8(0);
}
function deS16(de) {
    const n = deVarint(de, 16)

    return ((n >> 1) & 0xFFFF) ^ (-((n & 0b1) & 0xFFFF))
}
function deS32(de) {
    const n = deVarint(de, 32)

    return ((n >> 1) & 0xFFFFFFFF) ^ (-((n & 0b1) & 0xFFFFFFFF))
}
function deS64(de) {
    const n = deVarint(de, 64)

    return ((n >> 1) & 0xFFFFFFFFFFFFFFFF) ^ (-((n & 0b1) & 0xFFFFFFFFFFFFFFFF))
}
function deF32(de) {
    const view = new DataView(de.source.buffer, de.offset, 4);
    de.offset += 4;
    return view.getFloat32(0, true);
}
function deF64(de) {
    const view = new DataView(de.source.buffer, de.offset, 8);
    de.offset += 8;
    return view.getFloat64(0, true);
}
function deChar(de) {
    const sz = deU64(de);
    if (sz > 4) {
        throw new Error("Deserialize bad char");
    }
    const bytes = de.try_take_n(Number(sz));

    return __text_decoder.decode(bytes);
}
function deString(de) {
    const sz = deU64(de);

    let bytes = de.try_take_n(sz);

    return __text_decoder.decode(bytes);
}
function deBytes(de) {
    const sz = deU64(de);

    let bytes = de.try_take_n(Number(sz));

    return bytes;
}
function deOption(de, inner) {
    const tag = de.pop()

    switch (tag) {
        case 0:
            return null
        case 1: 
            return inner(de)
        default:
            throw new Error(`Deserialize bad option ${tag}`)
    }
}
function deResult(de, ok, err) {
    const tag = de.pop()

    switch (tag) {
        case 0:
            return { Ok: ok(de) }
        case 1: 
            return { Err: err(de) }
        default:
            throw new Error(`Deserialize bad result ${tag}`)
    }
}
function deList(de, inner) {
    const len = deU64(de);

    let out = [];

    for (let i = 0; i < len; i++) {
        out.push(inner(de));   
    }

    return out;
}
class Serializer {
    bytes;
    offset;
  
    constructor(len = 0) {
      this.bytes = new Uint8Array(len);
      this.offset = 0;
    }
  
    pushU8(num) {
      this.bytes[this.offset] = num
      this.offset += 1
    }
  
    push(bytes) {
      this.bytes.set(bytes, this.offset);
      this.offset += bytes.length;
    }

    filled() {
      return this.bytes.subarray(0, this.offset)
    }
  }
function serVarint(ser, bits, val) {
    for (let i = 0; i < varintMax[bits]; i++) {
        const buffer = new ArrayBuffer(bits / 8);
        const view = new DataView(buffer);
        view.setInt16(0, val, true);
        ser.bytes[ser.offset] = view.getUint8(0);
        if (val < 128) {
            ser.offset += 1
            return;
        }

        ser.bytes[ser.offset] |= 0x80;
        val >>= 7;
        ser.offset += 1
    }
}
function serBool(out, val) {
    out.pushU8(val === true ? 1 : 0)
}
function serU8(ser, val) {
    ser.pushU8(val)
}
function serU16(ser, val) {
    return serVarint(ser, 16, val)
}
function serU32(ser, val) {
    return serVarint(ser, 32, val)
}
function serU64(ser, val) {
    return serVarint(ser, 64, val)
}
function serS8(ser, val) {
    ser.pushU8(val)
}
function serS16(ser, val) {
    serVarint(ser, 16, (val << 1) ^ (val >> 15))
}
function serS32(ser, val) {
    serVarint(ser, 32, (val << 1) ^ (val >> 31))
}
function serS64(ser, val) {
    serVarint(ser, 64, (val << 1) ^ (val >> 63))
}
function serF32(ser, val) {
    const view = new DataView(ser.bytes.buffer, ser.offset, 4);
    ser.offset += 4;
    view.setFloat32(0, val, true);
}
function serF64(ser, val) {
    const view = new DataView(ser.bytes.buffer, ser.offset, 8);
    ser.offset += 8;
    view.setFloat64(0, val, true);
}
function serChar(ser, val) {
    if (val.len > 1) {
        throw new Error("Serialize bad char");
    }

    serU64(ser, val.length);

    ser.push(__text_encoder.encode(val))
}
function serString(ser, val) {
    serVarint(ser, 64, val.length)
    ser.push(__text_encoder.encode(val))
}
function serBytes(ser, val) {
    serU64(ser, val.length);
    ser.push(val)
}
function serOption(ser, inner, val) {
    ser.pushU8(val !== undefined && val !== null ? 1 : 0)
    if (val !== undefined && val !== null) {
        inner(ser, val)
    }
}
function serResult(ser, ok, err, val) {
    if (val.Ok) {
        serU8(ser, 0);
        return ok(ser, val.Ok);
    }

    if (val.Err) {
        serU8(ser, 1);
        return err(ser, val.Err);
    }

    throw new Error(`Serialize bad result ${val}`);
}
function serList(ser, inner, val) {
    serU64(ser, val.length)
    for (const el of val) {
        inner(ser, el)
    }
}
const __text_decoder = new TextDecoder('utf-8');
const __text_encoder = new TextEncoder();
function deEmpty(de) {
    return {
        
    }
}function deScalars(de) {
    return {
        a: deU32(de),
b: deU32(de)
    }
}function deReallyFlags(de) {
    return {
        a: deBool(de),
b: deBool(de),
c: deBool(de),
d: deBool(de),
e: deBool(de),
f: deBool(de),
g: deBool(de),
h: deBool(de),
i: deBool(de)
    }
}function deAggregates(de) {
    return {
        a: deScalars(de),
b: deU32(de),
c: deEmpty(de),
d: deString(de),
e: deReallyFlags(de)
    }
}function deFlag1(de) {
    return deU8(de)
}function deFlag2(de) {
    return deU8(de)
}function deFlag4(de) {
    return deU8(de)
}function deFlag8(de) {
    return deU8(de)
}function deFlag16(de) {
    return deU16(de)
}function deFlag32(de) {
    return deU32(de)
}function deFlag64(de) {
    return deU64(de)
}function deOtherRecord(de) {
    return {
        a1: deU32(de),
a2: deU64(de),
a3: deS32(de),
a4: deS64(de),
b: deString(de),
c: deBytes(de)
    }
}function deAllIntegers(de) {
    const tag = deU32(de)

    switch (tag) {
        case 0:
    return { Bool: deBool(de) }
case 1:
    return { U8: deU8(de) }
case 2:
    return { U16: deU16(de) }
case 3:
    return { U32: deU32(de) }
case 4:
    return { U64: deU64(de) }
case 5:
    return { I8: deS8(de) }
case 6:
    return { I16: deS16(de) }
case 7:
    return { S32: deS32(de) }
case 8:
    return { S64: deS64(de) }

        default:
            throw new Error(`unknown union case ${tag}`)
    }
}function deAllFloats(de) {
    const tag = deU32(de)

    switch (tag) {
        case 0:
    return { F32: deF32(de) }
case 1:
    return { F64: deF64(de) }

        default:
            throw new Error(`unknown union case ${tag}`)
    }
}function deAllText(de) {
    const tag = deU32(de)

    switch (tag) {
        case 0:
    return { Char: deChar(de) }
case 1:
    return { String: deString(de) }

        default:
            throw new Error(`unknown union case ${tag}`)
    }
}function deE1(de) {
    const tag = deU32(de)

    switch (tag) {
        case 0:
    return "A"

        default:
            throw new Error(`unknown enum case ${tag}`)
    }
}function deU1(de) {
    const tag = deU32(de)

    switch (tag) {
        case 0:
    return { U32: deU32(de) }
case 1:
    return { F32: deF32(de) }

        default:
            throw new Error(`unknown union case ${tag}`)
    }
}function deV1(de) {
    const tag = deU32(de)

    switch (tag) {
        case 0:
    return { A: null }
case 1:
    return { B: deU1(de) }
case 2:
    return { C: deE1(de) }
case 3:
    return { D: deString(de) }
case 4:
    return { E: deEmpty(de) }
case 5:
    return { F: null }
case 6:
    return { G: deU32(de) }

        default:
            throw new Error(`unknown variant case ${tag}`)
    }
}function serEmpty(ser, val) {
    
}function serScalars(ser, val) {
    serU32(ser, val.a),
serU32(ser, val.b)
}function serReallyFlags(ser, val) {
    serBool(ser, val.a),
serBool(ser, val.b),
serBool(ser, val.c),
serBool(ser, val.d),
serBool(ser, val.e),
serBool(ser, val.f),
serBool(ser, val.g),
serBool(ser, val.h),
serBool(ser, val.i)
}function serAggregates(ser, val) {
    serScalars(ser, val.a),
serU32(ser, val.b),
serEmpty(ser, val.c),
serString(ser, val.d),
serReallyFlags(ser, val.e)
}function serFlag1(ser, val) {
    return serU8(ser, val)
}function serFlag2(ser, val) {
    return serU8(ser, val)
}function serFlag4(ser, val) {
    return serU8(ser, val)
}function serFlag8(ser, val) {
    return serU8(ser, val)
}function serFlag16(ser, val) {
    return serU16(ser, val)
}function serFlag32(ser, val) {
    return serU32(ser, val)
}function serFlag64(ser, val) {
    return serU64(ser, val)
}function serOtherRecord(ser, val) {
    serU32(ser, val.a1),
serU64(ser, val.a2),
serS32(ser, val.a3),
serS64(ser, val.a4),
serString(ser, val.b),
serBytes(ser, val.c)
}function serSomeRecord(ser, val) {
    serString(ser, val.x),
serOtherRecord(ser, val.y),
serList(ser, (ser, v) => serOtherRecord(ser, v), val.z),
serU32(ser, val.c1),
serU64(ser, val.c2),
serS32(ser, val.c3),
serS64(ser, val.c4)
}function serAllIntegers(ser, val) {
    if (val.Bool) {
    serU32(ser, 0);
    return serBool(ser, val.Bool)
}
                if (val.U8) {
    serU32(ser, 1);
    return serU8(ser, val.U8)
}
                if (val.U16) {
    serU32(ser, 2);
    return serU16(ser, val.U16)
}
                if (val.U32) {
    serU32(ser, 3);
    return serU32(ser, val.U32)
}
                if (val.U64) {
    serU32(ser, 4);
    return serU64(ser, val.U64)
}
                if (val.I8) {
    serU32(ser, 5);
    return serS8(ser, val.I8)
}
                if (val.I16) {
    serU32(ser, 6);
    return serS16(ser, val.I16)
}
                if (val.S32) {
    serU32(ser, 7);
    return serS32(ser, val.S32)
}
                if (val.S64) {
    serU32(ser, 8);
    return serS64(ser, val.S64)
}
                

    throw new Error("unknown union case")
}function serAllFloats(ser, val) {
    if (val.F32) {
    serU32(ser, 0);
    return serF32(ser, val.F32)
}
                if (val.F64) {
    serU32(ser, 1);
    return serF64(ser, val.F64)
}
                

    throw new Error("unknown union case")
}function serAllText(ser, val) {
    if (val.Char) {
    serU32(ser, 0);
    return serChar(ser, val.Char)
}
                if (val.String) {
    serU32(ser, 1);
    return serString(ser, val.String)
}
                

    throw new Error("unknown union case")
}function serE1(ser, val) {
    switch (val) {
        case "A":
    serU32(ser, 0)
    return

        default:
            throw new Error("unknown enum case")
    }
}function serU1(ser, val) {
    if (val.U32) {
    serU32(ser, 0);
    return serU32(ser, val.U32)
}
                if (val.F32) {
    serU32(ser, 1);
    return serF32(ser, val.F32)
}
                

    throw new Error("unknown union case")
}function serV1(ser, val) {
    if (val.A) {
    serU32(ser, 0);
    return 
}
if (val.B) {
    serU32(ser, 1);
    return serU1(ser, val.B)
}
if (val.C) {
    serU32(ser, 2);
    return serE1(ser, val.C)
}
if (val.D) {
    serU32(ser, 3);
    return serString(ser, val.D)
}
if (val.E) {
    serU32(ser, 4);
    return serEmpty(ser, val.E)
}
if (val.F) {
    serU32(ser, 5);
    return 
}
if (val.G) {
    serU32(ser, 6);
    return serU32(ser, val.G)
}


    throw new Error("unknown variant case")
}

/**
* @param {Empty} x 
* @returns {Promise<Empty>} 
*/
export async function empty (x) {
    const ser = new Serializer(0)
    serEmpty(ser, x)

    return fetch('ipc://localhost/roundtrip/empty', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deEmpty(de)
        })
}

/**
* @param {Scalars} val 
* @returns {Promise<Scalars>} 
*/
export async function recordScalars (val) {
    const ser = new Serializer(5+5)
    serScalars(ser, val)

    return fetch('ipc://localhost/roundtrip/record_scalars', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deScalars(de)
        })
}

/**
* @param {ReallyFlags} val 
* @returns {Promise<ReallyFlags>} 
*/
export async function recordReallyFlags (val) {
    const ser = new Serializer(1+1+1+1+1+1+1+1+1)
    serReallyFlags(ser, val)

    return fetch('ipc://localhost/roundtrip/record_really_flags', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deReallyFlags(de)
        })
}

/**
* @param {Aggregates} val 
* @returns {Promise<Aggregates>} 
*/
export async function recordAggregates (val) {
    const ser = new Serializer(5+5+5+0+val.d.length * 4 + 10+1+1+1+1+1+1+1+1+1)
    serAggregates(ser, val)

    return fetch('ipc://localhost/roundtrip/record_aggregates', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deAggregates(de)
        })
}

/**
* @param {Flag1} x 
* @returns {Promise<Flag1>} 
*/
export async function flag1 (x) {
    const ser = new Serializer(1)
    serFlag1(ser, x)

    return fetch('ipc://localhost/roundtrip/flag1', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deFlag1(de)
        })
}

/**
* @param {Flag2} x 
* @returns {Promise<Flag2>} 
*/
export async function flag2 (x) {
    const ser = new Serializer(1)
    serFlag2(ser, x)

    return fetch('ipc://localhost/roundtrip/flag2', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deFlag2(de)
        })
}

/**
* @param {Flag4} x 
* @returns {Promise<Flag4>} 
*/
export async function flag4 (x) {
    const ser = new Serializer(1)
    serFlag4(ser, x)

    return fetch('ipc://localhost/roundtrip/flag4', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deFlag4(de)
        })
}

/**
* @param {Flag8} x 
* @returns {Promise<Flag8>} 
*/
export async function flag8 (x) {
    const ser = new Serializer(1)
    serFlag8(ser, x)

    return fetch('ipc://localhost/roundtrip/flag8', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deFlag8(de)
        })
}

/**
* @param {Flag16} x 
* @returns {Promise<Flag16>} 
*/
export async function flag16 (x) {
    const ser = new Serializer(3)
    serFlag16(ser, x)

    return fetch('ipc://localhost/roundtrip/flag16', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deFlag16(de)
        })
}

/**
* @param {Flag32} x 
* @returns {Promise<Flag32>} 
*/
export async function flag32 (x) {
    const ser = new Serializer(5)
    serFlag32(ser, x)

    return fetch('ipc://localhost/roundtrip/flag32', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deFlag32(de)
        })
}

/**
* @param {Flag64} x 
* @returns {Promise<Flag64>} 
*/
export async function flag64 (x) {
    const ser = new Serializer(10)
    serFlag64(ser, x)

    return fetch('ipc://localhost/roundtrip/flag64', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deFlag64(de)
        })
}

/**
* @param {number} x 
* @returns {Promise<number>} 
*/
export async function float32 (x) {
    const ser = new Serializer(4)
    serF32(ser, x)

    return fetch('ipc://localhost/roundtrip/float32', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deF32(de)
        })
}

/**
* @param {number} x 
* @returns {Promise<number>} 
*/
export async function float64 (x) {
    const ser = new Serializer(8)
    serF64(ser, x)

    return fetch('ipc://localhost/roundtrip/float64', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deF64(de)
        })
}

/**
* @param {number} x 
* @returns {Promise<number>} 
*/
export async function u8 (x) {
    const ser = new Serializer(1)
    serU8(ser, x)

    return fetch('ipc://localhost/roundtrip/u8', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deU8(de)
        })
}

/**
* @param {number} x 
* @returns {Promise<number>} 
*/
export async function s8 (x) {
    const ser = new Serializer(1)
    serS8(ser, x)

    return fetch('ipc://localhost/roundtrip/s8', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deS8(de)
        })
}

/**
* @param {number} x 
* @returns {Promise<number>} 
*/
export async function u16 (x) {
    const ser = new Serializer(3)
    serU16(ser, x)

    return fetch('ipc://localhost/roundtrip/u16', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deU16(de)
        })
}

/**
* @param {number} x 
* @returns {Promise<number>} 
*/
export async function s16 (x) {
    const ser = new Serializer(3)
    serS16(ser, x)

    return fetch('ipc://localhost/roundtrip/s16', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deS16(de)
        })
}

/**
* @param {number} x 
* @returns {Promise<number>} 
*/
export async function u32 (x) {
    const ser = new Serializer(5)
    serU32(ser, x)

    return fetch('ipc://localhost/roundtrip/u32', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deU32(de)
        })
}

/**
* @param {number} x 
* @returns {Promise<number>} 
*/
export async function s32 (x) {
    const ser = new Serializer(5)
    serS32(ser, x)

    return fetch('ipc://localhost/roundtrip/s32', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deS32(de)
        })
}

/**
* @param {bigint} x 
* @returns {Promise<bigint>} 
*/
export async function u64 (x) {
    const ser = new Serializer(10)
    serU64(ser, x)

    return fetch('ipc://localhost/roundtrip/u64', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deU64(de)
        })
}

/**
* @param {bigint} x 
* @returns {Promise<bigint>} 
*/
export async function s64 (x) {
    const ser = new Serializer(10)
    serS64(ser, x)

    return fetch('ipc://localhost/roundtrip/s64', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deS64(de)
        })
}

/**
* @param {Uint8Array[]} x 
* @returns {Promise<Uint8Array[]>} 
*/
export async function listU8 (x) {
    const ser = new Serializer(x.length + 10)
    serBytes(ser, x)

    return fetch('ipc://localhost/roundtrip/list_u8', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deBytes(de)
        })
}

/**
* @param {Uint16Array[]} x 
* @returns {Promise<Uint16Array[]>} 
*/
export async function listU16 (x) {
    const ser = new Serializer(x.reduce((acc, cur) => acc + 3, 0) + 10)
    serList(ser, (ser, v) => serU16(ser, v), x)

    return fetch('ipc://localhost/roundtrip/list_u16', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deList(de, (de) => deU16(de))
        })
}

/**
* @param {Uint32Array[]} x 
* @returns {Promise<Uint32Array[]>} 
*/
export async function listU32 (x) {
    const ser = new Serializer(x.reduce((acc, cur) => acc + 5, 0) + 10)
    serList(ser, (ser, v) => serU32(ser, v), x)

    return fetch('ipc://localhost/roundtrip/list_u32', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deList(de, (de) => deU32(de))
        })
}

/**
* @param {BigUint64Array[]} x 
* @returns {Promise<BigUint64Array[]>} 
*/
export async function listU64 (x) {
    const ser = new Serializer(x.reduce((acc, cur) => acc + 10, 0) + 10)
    serList(ser, (ser, v) => serU64(ser, v), x)

    return fetch('ipc://localhost/roundtrip/list_u64', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deList(de, (de) => deU64(de))
        })
}

/**
* @param {Int8Array[]} x 
* @returns {Promise<Int8Array[]>} 
*/
export async function listS8 (x) {
    const ser = new Serializer(x.length + 10)
    serList(ser, (ser, v) => serS8(ser, v), x)

    return fetch('ipc://localhost/roundtrip/list_s8', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deList(de, (de) => deS8(de))
        })
}

/**
* @param {Int16Array[]} x 
* @returns {Promise<Int16Array[]>} 
*/
export async function listS16 (x) {
    const ser = new Serializer(x.reduce((acc, cur) => acc + 3, 0) + 10)
    serList(ser, (ser, v) => serS16(ser, v), x)

    return fetch('ipc://localhost/roundtrip/list_s16', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deList(de, (de) => deS16(de))
        })
}

/**
* @param {Int32Array[]} x 
* @returns {Promise<Int32Array[]>} 
*/
export async function listS32 (x) {
    const ser = new Serializer(x.reduce((acc, cur) => acc + 5, 0) + 10)
    serList(ser, (ser, v) => serS32(ser, v), x)

    return fetch('ipc://localhost/roundtrip/list_s32', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deList(de, (de) => deS32(de))
        })
}

/**
* @param {BigInt64Array[]} x 
* @returns {Promise<BigInt64Array[]>} 
*/
export async function listS64 (x) {
    const ser = new Serializer(x.reduce((acc, cur) => acc + 10, 0) + 10)
    serList(ser, (ser, v) => serS64(ser, v), x)

    return fetch('ipc://localhost/roundtrip/list_s64', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deList(de, (de) => deS64(de))
        })
}

/**
* @param {Float32Array[]} x 
* @returns {Promise<Float32Array[]>} 
*/
export async function listFloat32 (x) {
    const ser = new Serializer(x.reduce((acc, cur) => acc + 4, 0) + 10)
    serList(ser, (ser, v) => serF32(ser, v), x)

    return fetch('ipc://localhost/roundtrip/list_float32', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deList(de, (de) => deF32(de))
        })
}

/**
* @param {Float64Array[]} x 
* @returns {Promise<Float64Array[]>} 
*/
export async function listFloat64 (x) {
    const ser = new Serializer(x.reduce((acc, cur) => acc + 8, 0) + 10)
    serList(ser, (ser, v) => serF64(ser, v), x)

    return fetch('ipc://localhost/roundtrip/list_float64', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deList(de, (de) => deF64(de))
        })
}

/**
* @param {[number, number][]} x 
* @returns {Promise<[number, number][]>} 
*/
export async function tupleList (x) {
    const ser = new Serializer(x.reduce((acc, cur) => acc + 1+1, 0) + 10)
    serList(ser, (ser, v) => {serU8(ser, v[0]);serS8(ser, v[1])}, x)

    return fetch('ipc://localhost/roundtrip/tuple_list', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deList(de, (de) => [deU8(de), deS8(de)])
        })
}

/**
* @param {string[]} a 
* @returns {Promise<string[]>} 
*/
export async function stringList (a) {
    const ser = new Serializer(a.reduce((acc, cur) => acc + cur.length * 4 + 10, 0) + 10)
    serList(ser, (ser, v) => serString(ser, v), a)

    return fetch('ipc://localhost/roundtrip/string_list', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deList(de, (de) => deString(de))
        })
}

/**
* @param {[number, string][]} x 
* @returns {Promise<[number, string][]>} 
*/
export async function tupleStringList (x) {
    const ser = new Serializer(x.reduce((acc, cur) => acc + 1+cur[1].length * 4 + 10, 0) + 10)
    serList(ser, (ser, v) => {serU8(ser, v[0]);serString(ser, v[1])}, x)

    return fetch('ipc://localhost/roundtrip/tuple_string_list', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deList(de, (de) => [deU8(de), deString(de)])
        })
}

/**
* @param {SomeRecord[]} x 
* @returns {Promise<OtherRecord[]>} 
*/
export async function recordList (x) {
    const ser = new Serializer(x.reduce((acc, cur) => acc + cur.x.length * 4 + 10+5+10+5+10+cur.y.b.length * 4 + 10+cur.y.c.length + 10+cur.z.reduce((acc, cur) => acc + 5+10+5+10+cur.b.length * 4 + 10+cur.c.length + 10, 0) + 10+5+10+5+10, 0) + 10)
    serList(ser, (ser, v) => serSomeRecord(ser, v), x)

    return fetch('ipc://localhost/roundtrip/record_list', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deList(de, (de) => deOtherRecord(de))
        })
}

/**
* @param {AllIntegers} x 
* @returns {Promise<AllIntegers>} 
*/
export async function allIntegers (x) {
    const ser = new Serializer(Math.max(x.Bool ? 1 : 0,x.U8 ? 1 : 0,x.U16 ? 3 : 0,x.U32 ? 5 : 0,x.U64 ? 10 : 0,x.I8 ? 1 : 0,x.I16 ? 3 : 0,x.S32 ? 5 : 0,x.S64 ? 10 : 0) + 5)
    serAllIntegers(ser, x)

    return fetch('ipc://localhost/roundtrip/all_integers', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deAllIntegers(de)
        })
}

/**
* @param {AllFloats} x 
* @returns {Promise<AllFloats>} 
*/
export async function allFloats (x) {
    const ser = new Serializer(Math.max(x.F32 ? 4 : 0,x.F64 ? 8 : 0) + 5)
    serAllFloats(ser, x)

    return fetch('ipc://localhost/roundtrip/all_floats', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deAllFloats(de)
        })
}

/**
* @param {AllText} x 
* @returns {Promise<AllText>} 
*/
export async function allText (x) {
    const ser = new Serializer(Math.max(x.Char ? 4 : 0,x.String ? x.String.length * 4 + 10 : 0) + 5)
    serAllText(ser, x)

    return fetch('ipc://localhost/roundtrip/all_text', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deAllText(de)
        })
}

/**
* @param {E1} x 
* @returns {Promise<E1>} 
*/
export async function e1 (x) {
    const ser = new Serializer(5)
    serE1(ser, x)

    return fetch('ipc://localhost/roundtrip/e1', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deE1(de)
        })
}

/**
* @param {V1} x 
* @returns {Promise<V1>} 
*/
export async function v1 (x) {
    const ser = new Serializer(Math.max(x.b ? Math.max(x.b.U32 ? 5 : 0,x.b.F32 ? 4 : 0) + 5 : 0,x.c ? 5 : 0,x.d ? x.d.length * 4 + 10 : 0,x.e ? 0 : 0,x.g ? 5 : 0) + 5)
    serV1(ser, x)

    return fetch('ipc://localhost/roundtrip/v1', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deV1(de)
        })
}

/**
* @param {boolean | null} a 
* @param {[] | null} b 
* @param {number | null} c 
* @param {E1 | null} d 
* @param {number | null} e 
* @param {U1 | null} f 
* @param {boolean | null | null} g 
* @returns {Promise<[boolean | null, [] | null, number | null, E1 | null, number | null, U1 | null, boolean | null | null]>} 
*/
export async function options (a, b, c, d, e, f, g) {
    const ser = new Serializer(1 + 1+0 + 1+5 + 1+5 + 1+4 + 1+Math.max(f.U32 ? 5 : 0,f.F32 ? 4 : 0) + 5 + 1+1 + 1 + 1)
    serOption(ser, (ser, v) => serBool(ser, v), a);
serOption(ser, (ser, v) => {}, b);
serOption(ser, (ser, v) => serU32(ser, v), c);
serOption(ser, (ser, v) => serE1(ser, v), d);
serOption(ser, (ser, v) => serF32(ser, v), e);
serOption(ser, (ser, v) => serU1(ser, v), f);
serOption(ser, (ser, v) => serOption(ser, (ser, v) => serBool(ser, v), v), g)

    return fetch('ipc://localhost/roundtrip/options', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return [deOption(de, (de) => deBool(de)), deOption(de, (de) => []), deOption(de, (de) => deU32(de)), deOption(de, (de) => deE1(de)), deOption(de, (de) => deF32(de)), deOption(de, (de) => deU1(de)), deOption(de, (de) => deOption(de, (de) => deBool(de)))]
        })
}

/**
* @param {Result<_, _>} a 
* @param {Result<_, E1>} b 
* @param {Result<E1, _>} c 
* @param {Result<[], []>} d 
* @param {Result<number, V1>} e 
* @param {Result<string, Uint8Array[]>} f 
* @returns {Promise<[Result<_, _>, Result<_, E1>, Result<E1, _>, Result<[], []>, Result<number, V1>, Result<string, Uint8Array[]>]>} 
*/
export async function results (a, b, c, d, e, f) {
    const ser = new Serializer(0 + 0 + 1+0 + 5 + 1+5 + 0 + 1+0 + 0 + 1+5 + Math.max(e.Err.b ? Math.max(e.Err.b.U32 ? 5 : 0,e.Err.b.F32 ? 4 : 0) + 5 : 0,e.Err.c ? 5 : 0,e.Err.d ? e.Err.d.length * 4 + 10 : 0,e.Err.e ? 0 : 0,e.Err.g ? 5 : 0) + 5 + 1+f.Ok.length * 4 + 10 + f.Err.length + 10 + 1)
    serResult(ser, (ser, v) => {}, (ser, v) => {}, a);
serResult(ser, (ser, v) => {}, (ser, v) => serE1(ser, v), b);
serResult(ser, (ser, v) => serE1(ser, v), (ser, v) => {}, c);
serResult(ser, (ser, v) => {}, (ser, v) => {}, d);
serResult(ser, (ser, v) => serU32(ser, v), (ser, v) => serV1(ser, v), e);
serResult(ser, (ser, v) => serString(ser, v), (ser, v) => serBytes(ser, v), f)

    return fetch('ipc://localhost/roundtrip/results', { method: "POST", body: ser.filled(), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return [deResult(de, () => {}, () => {}), deResult(de, () => {}, deE1(de)), deResult(de, deE1(de), () => {}), deResult(de, [], []), deResult(de, deU32(de), deV1(de)), deResult(de, deString(de), deBytes(de))]
        })
}

