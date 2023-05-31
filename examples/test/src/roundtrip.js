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
function max_of_last_byte(type) {
  let extra_bits = type % 7;
  return (1 << extra_bits) - 1;
}

function de_varint(de, bits) {
  let out = 0;

  for (let i = 0; i < varint_max[bits]; i++) {
    const val = de.pop();
    const carry = val & 0x7F;
    out |= carry << (7 * i);

    if ((val & 0x80) === 0) {
      if (i === varint_max[bits] - 1 && val > max_of_last_byte(bits)) {
        throw new Error('deserialize bad variant')
      } else {
        return out
      }
    }
  }

  throw new Error('deserialize bad variant')
}

function de_varint_big(de, bits) {
  let out = 0n;

  for (let i = 0; i < varint_max[bits]; i++) {
    const val = de.pop();
    const carry = BigInt(val) & 0x7Fn;
    out |= carry << (7n * BigInt(i));

    if ((val & 0x80) === 0) {
      if (i === varint_max[bits] - 1 && val > max_of_last_byte(bits)) {
        throw new Error('deserialize bad variant')
      } else {
        return out
      }
    }
  }

  throw new Error('deserialize bad variant')
}
function deserializeBool(de) {
    const val = de.pop();

    return val != 0
}
function deserializeU8(de) {
    return de.pop()
}
function deserializeU16(de) {
    return de_varint(de, 16)
}
function deserializeU32(de) {
    return de_varint(de, 32)
}
function deserializeU64(de) {
  return de_varint_big(de, 64)
}
function deserializeU128(de) {
  return de_varint_big(de, 128)
}
function deserializeS8(de) {
    const buf = new ArrayBuffer(1);
    const view = new DataView(buf);

    buf[0] = view.setUint8(0, de.pop());

    return view.getInt8(0);
}
function deserializeS16(de) {
    const n = de_varint(de, 16)

    return Number(((n >> 1) & 0xFFFF) ^ (-((n & 0b1) & 0xFFFF)))
}
function deserializeS32(de) {
    const n = de_varint(de, 32)

    return Number(((n >> 1) & 0xFFFFFFFF) ^ (-((n & 0b1) & 0xFFFFFFFF)))
}
function deserializeS64(de) {
  const n = de_varint_big(de, 64)

  return ((n >> 1n) & 0xFFFFFFFFFFFFFFFFn) ^ (-((n & 0b1n) & 0xFFFFFFFFFFFFFFFFn))
}
function deserializeS128(de) {
  const n = de_varint_big(de, 128)

  return ((n >> 1n) & 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFn) ^ (-((n & 0b1n) & 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFn))
}
function deserializeF32(de) {
    const bytes = de.try_take_n(4);

    const buf = new ArrayBuffer(4);
    const view = new DataView(buf);

    bytes.forEach((v, i) => view.setUint8(i, v));

    return view.getFloat32(0, true);
}
function deserializeF64(de) {
    const bytes = de.try_take_n(8);

    const buf = new ArrayBuffer(8);
    const view = new DataView(buf);

    bytes.forEach((v, i) => view.setUint8(i, v));

    return view.getFloat64(0, true);
}
function deserializeChar(de) {
    const sz = deserializeU64(de);
    if (sz > 4) {
        throw new Error("Deserialize bad char");
    }
    const bytes = de.try_take_n(Number(sz));

    return __text_decoder.decode(bytes);
}
function deserializeString(de) {
    const sz = deserializeU64(de);

    let bytes = de.try_take_n(Number(sz));

    return __text_decoder.decode(bytes);
}
function deserializeBytes(de) {
    const sz = deserializeU64(de);

    let bytes = de.try_take_n(Number(sz));

    return bytes;
}
function deserializeOption(de, inner) {
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
function deserializeResult(de, ok, err) {
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
function deserializeList(de, inner) {
    const len = deserializeU64(de);

    let out = [];

    for (let i = 0; i < len; i++) {
        out.push(inner(de));   
    }

    return out;
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
function serializeBool(out, val) {
    out.push(val === true ? 1 : 0)
}
function serializeU8(out, val) {
    return out.push(val)
}
function serializeU16(out, val) {
    return ser_varint(out, 16, val)
}
function serializeU32(out, val) {
    return ser_varint(out, 32, val)
}
function serializeU64(out, val) {
  return ser_varint_big(out, 64, BigInt(val))
}
function serializeU128(out, val) {
  return ser_varint_big(out, 128, BigInt(val))
}
function serializeS8(out, val) {
    out.push(val)
}
function serializeS16(out, val) {
    ser_varint(out, 16, (val << 1) ^ (val >> 15))
}
function serializeS32(out, val) {
    ser_varint(out, 32, (val << 1) ^ (val >> 31))
}
function serializeS64(out, val) {
  val = BigInt(val)
  ser_varint_big(out, 64, (val << 1n) ^ (val >> 63n))
}
function serializeS128(out, val) {
  val = BigInt(val)
  ser_varint_big(out, 128, (val << 1n) ^ (val >> 127n))
}
function serializeF32(out, val) {
    const buf = new ArrayBuffer(4);
    const view = new DataView(buf);

    view.setFloat32(0, val, true);

    out.push(...new Uint8Array(buf))
}
function serializeF64(out, val) {
    const buf = new ArrayBuffer(8);
    const view = new DataView(buf);

    view.setFloat64(0, val, true);

    out.push(...new Uint8Array(buf))
}
function serializeChar(out, val) {
    if (val.len > 1) {
        throw new Error("Serialize bad char");
    }

    serializeU64(out, val.length);

    out.push(...__text_encoder.encode(val))
}
function serializeString(out, val) {
    serializeU64(out, val.length);

    out.push(...__text_encoder.encode(val))
}
function serializeBytes(out, val) {
    serializeU64(out, val.length);
    out.push(...val)
}
function serializeOption(out, inner, val) {
    serializeU8(out, !!val ? 1 : 0)
    if (val) {
        inner(out, val)
    }
}
function serializeResult(out, ok, err, val) {
    if (val.Ok) {
        serializeU8(out, 0);
        return ok(out, val.Ok);
    }

    if (val.Err) {
        serializeU8(out, 1);
        return err(out, val.Err);
    }

    throw new Error(`Serialize bad result ${val}`);
}
function serializeList(out, inner, val) {
    serializeU64(out, val.length)
    for (const el of val) {
        inner(out, el)
    }
}
const __text_decoder = new TextDecoder('utf-8');
const __text_encoder = new TextEncoder();
function deserializeEmpty(de) {
    return {
        
    }
}function deserializeScalars(de) {
    return {
        a: deserializeU32(de),
b: deserializeU32(de)
    }
}function deserializeReallyFlags(de) {
    return {
        a: deserializeBool(de),
b: deserializeBool(de),
c: deserializeBool(de),
d: deserializeBool(de),
e: deserializeBool(de),
f: deserializeBool(de),
g: deserializeBool(de),
h: deserializeBool(de),
i: deserializeBool(de)
    }
}function deserializeAggregates(de) {
    return {
        a: deserializeScalars(de),
b: deserializeU32(de),
c: deserializeEmpty(de),
d: deserializeString(de),
e: deserializeReallyFlags(de)
    }
}function deserializeFlag1(de) {
    return deserializeU8(de)
}function deserializeFlag2(de) {
    return deserializeU8(de)
}function deserializeFlag4(de) {
    return deserializeU8(de)
}function deserializeFlag8(de) {
    return deserializeU8(de)
}function deserializeFlag16(de) {
    return deserializeU16(de)
}function deserializeFlag32(de) {
    return deserializeU32(de)
}function deserializeFlag64(de) {
    return deserializeU64(de)
}function deserializeOtherRecord(de) {
    return {
        a1: deserializeU32(de),
a2: deserializeU64(de),
a3: deserializeS32(de),
a4: deserializeS64(de),
b: deserializeString(de),
c: deserializeBytes(de)
    }
}function deserializeAllIntegers(de) {
    const tag = deserializeU32(de)

    switch (tag) {
        case 0:
    return { Bool: deserializeBool(de) }
case 1:
    return { U8: deserializeU8(de) }
case 2:
    return { U16: deserializeU16(de) }
case 3:
    return { U32: deserializeU32(de) }
case 4:
    return { U64: deserializeU64(de) }
case 5:
    return { I8: deserializeS8(de) }
case 6:
    return { I16: deserializeS16(de) }
case 7:
    return { S32: deserializeS32(de) }
case 8:
    return { S64: deserializeS64(de) }

        default:
            throw new Error(`unknown union case ${tag}`)
    }
}function deserializeAllFloats(de) {
    const tag = deserializeU32(de)

    switch (tag) {
        case 0:
    return { F32: deserializeF32(de) }
case 1:
    return { F64: deserializeF64(de) }

        default:
            throw new Error(`unknown union case ${tag}`)
    }
}function deserializeAllText(de) {
    const tag = deserializeU32(de)

    switch (tag) {
        case 0:
    return { Char: deserializeChar(de) }
case 1:
    return { String: deserializeString(de) }

        default:
            throw new Error(`unknown union case ${tag}`)
    }
}function deserializeE1(de) {
    const tag = deserializeU32(de)

    switch (tag) {
        case 0:
    return "A"

        default:
            throw new Error(`unknown enum case ${tag}`)
    }
}function deserializeU1(de) {
    const tag = deserializeU32(de)

    switch (tag) {
        case 0:
    return { U32: deserializeU32(de) }
case 1:
    return { F32: deserializeF32(de) }

        default:
            throw new Error(`unknown union case ${tag}`)
    }
}function deserializeV1(de) {
    const tag = deserializeU32(de)

    switch (tag) {
        case 0:
    return { A: null }
case 1:
    return { B: deserializeU1(de) }
case 2:
    return { C: deserializeE1(de) }
case 3:
    return { D: deserializeString(de) }
case 4:
    return { E: deserializeEmpty(de) }
case 5:
    return { F: null }
case 6:
    return { G: deserializeU32(de) }

        default:
            throw new Error(`unknown variant case ${tag}`)
    }
}function serializeEmpty(out, val) {
    
}function serializeScalars(out, val) {
    serializeU32(out, val.a),
serializeU32(out, val.b)
}function serializeReallyFlags(out, val) {
    serializeBool(out, val.a),
serializeBool(out, val.b),
serializeBool(out, val.c),
serializeBool(out, val.d),
serializeBool(out, val.e),
serializeBool(out, val.f),
serializeBool(out, val.g),
serializeBool(out, val.h),
serializeBool(out, val.i)
}function serializeAggregates(out, val) {
    serializeScalars(out, val.a),
serializeU32(out, val.b),
serializeEmpty(out, val.c),
serializeString(out, val.d),
serializeReallyFlags(out, val.e)
}function serializeFlag1(out, val) {
    return serializeU8(out, val)
}function serializeFlag2(out, val) {
    return serializeU8(out, val)
}function serializeFlag4(out, val) {
    return serializeU8(out, val)
}function serializeFlag8(out, val) {
    return serializeU8(out, val)
}function serializeFlag16(out, val) {
    return serializeU16(out, val)
}function serializeFlag32(out, val) {
    return serializeU32(out, val)
}function serializeFlag64(out, val) {
    return serializeU64(out, val)
}function serializeOtherRecord(out, val) {
    serializeU32(out, val.a1),
serializeU64(out, val.a2),
serializeS32(out, val.a3),
serializeS64(out, val.a4),
serializeString(out, val.b),
serializeBytes(out, val.c)
}function serializeSomeRecord(out, val) {
    serializeString(out, val.x),
serializeOtherRecord(out, val.y),
serializeList(out, (out, v) => serializeOtherRecord(out, v), val.z),
serializeU32(out, val.c1),
serializeU64(out, val.c2),
serializeS32(out, val.c3),
serializeS64(out, val.c4)
}function serializeAllIntegers(out, val) {
    if (val.Bool) {
    serializeU32(out, 0);
    return serializeBool(out, val.Bool)
}
                if (val.U8) {
    serializeU32(out, 1);
    return serializeU8(out, val.U8)
}
                if (val.U16) {
    serializeU32(out, 2);
    return serializeU16(out, val.U16)
}
                if (val.U32) {
    serializeU32(out, 3);
    return serializeU32(out, val.U32)
}
                if (val.U64) {
    serializeU32(out, 4);
    return serializeU64(out, val.U64)
}
                if (val.I8) {
    serializeU32(out, 5);
    return serializeS8(out, val.I8)
}
                if (val.I16) {
    serializeU32(out, 6);
    return serializeS16(out, val.I16)
}
                if (val.S32) {
    serializeU32(out, 7);
    return serializeS32(out, val.S32)
}
                if (val.S64) {
    serializeU32(out, 8);
    return serializeS64(out, val.S64)
}
                

    throw new Error("unknown union case")
}function serializeAllFloats(out, val) {
    if (val.F32) {
    serializeU32(out, 0);
    return serializeF32(out, val.F32)
}
                if (val.F64) {
    serializeU32(out, 1);
    return serializeF64(out, val.F64)
}
                

    throw new Error("unknown union case")
}function serializeAllText(out, val) {
    if (val.Char) {
    serializeU32(out, 0);
    return serializeChar(out, val.Char)
}
                if (val.String) {
    serializeU32(out, 1);
    return serializeString(out, val.String)
}
                

    throw new Error("unknown union case")
}function serializeE1(out, val) {
    switch (val) {
        case "A":
    serializeU32(out, 0)
    return

        default:
            throw new Error("unknown enum case")
    }
}function serializeU1(out, val) {
    if (val.U32) {
    serializeU32(out, 0);
    return serializeU32(out, val.U32)
}
                if (val.F32) {
    serializeU32(out, 1);
    return serializeF32(out, val.F32)
}
                

    throw new Error("unknown union case")
}function serializeV1(out, val) {
    if (val.A) {
    serializeU32(out, 0);
    return 
}
if (val.B) {
    serializeU32(out, 1);
    return serializeU1(out, val.B)
}
if (val.C) {
    serializeU32(out, 2);
    return serializeE1(out, val.C)
}
if (val.D) {
    serializeU32(out, 3);
    return serializeString(out, val.D)
}
if (val.E) {
    serializeU32(out, 4);
    return serializeEmpty(out, val.E)
}
if (val.F) {
    serializeU32(out, 5);
    return 
}
if (val.G) {
    serializeU32(out, 6);
    return serializeU32(out, val.G)
}


    throw new Error("unknown variant case")
}

/**
* @param {Empty} x 
* @returns {Promise<Empty>} 
*/
export async function empty (x) {
    const out = []
    serializeEmpty(out, x)

    return fetch('ipc://localhost/roundtrip/empty', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeEmpty(de)
        })
}

/**
* @param {Scalars} val 
* @returns {Promise<Scalars>} 
*/
export async function recordScalars (val) {
    const out = []
    serializeScalars(out, val)

    return fetch('ipc://localhost/roundtrip/record_scalars', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeScalars(de)
        })
}

/**
* @param {ReallyFlags} val 
* @returns {Promise<ReallyFlags>} 
*/
export async function recordReallyFlags (val) {
    const out = []
    serializeReallyFlags(out, val)

    return fetch('ipc://localhost/roundtrip/record_really_flags', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeReallyFlags(de)
        })
}

/**
* @param {Aggregates} val 
* @returns {Promise<Aggregates>} 
*/
export async function recordAggregates (val) {
    const out = []
    serializeAggregates(out, val)

    return fetch('ipc://localhost/roundtrip/record_aggregates', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeAggregates(de)
        })
}

/**
* @param {Flag1} x 
* @returns {Promise<Flag1>} 
*/
export async function flag1 (x) {
    const out = []
    serializeFlag1(out, x)

    return fetch('ipc://localhost/roundtrip/flag1', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeFlag1(de)
        })
}

/**
* @param {Flag2} x 
* @returns {Promise<Flag2>} 
*/
export async function flag2 (x) {
    const out = []
    serializeFlag2(out, x)

    return fetch('ipc://localhost/roundtrip/flag2', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeFlag2(de)
        })
}

/**
* @param {Flag4} x 
* @returns {Promise<Flag4>} 
*/
export async function flag4 (x) {
    const out = []
    serializeFlag4(out, x)

    return fetch('ipc://localhost/roundtrip/flag4', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeFlag4(de)
        })
}

/**
* @param {Flag8} x 
* @returns {Promise<Flag8>} 
*/
export async function flag8 (x) {
    const out = []
    serializeFlag8(out, x)

    return fetch('ipc://localhost/roundtrip/flag8', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeFlag8(de)
        })
}

/**
* @param {Flag16} x 
* @returns {Promise<Flag16>} 
*/
export async function flag16 (x) {
    const out = []
    serializeFlag16(out, x)

    return fetch('ipc://localhost/roundtrip/flag16', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeFlag16(de)
        })
}

/**
* @param {Flag32} x 
* @returns {Promise<Flag32>} 
*/
export async function flag32 (x) {
    const out = []
    serializeFlag32(out, x)

    return fetch('ipc://localhost/roundtrip/flag32', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeFlag32(de)
        })
}

/**
* @param {Flag64} x 
* @returns {Promise<Flag64>} 
*/
export async function flag64 (x) {
    const out = []
    serializeFlag64(out, x)

    return fetch('ipc://localhost/roundtrip/flag64', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeFlag64(de)
        })
}

/**
* @param {number} x 
* @returns {Promise<number>} 
*/
export async function float32 (x) {
    const out = []
    serializeF32(out, x)

    return fetch('ipc://localhost/roundtrip/float32', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeF32(de)
        })
}

/**
* @param {number} x 
* @returns {Promise<number>} 
*/
export async function float64 (x) {
    const out = []
    serializeF64(out, x)

    return fetch('ipc://localhost/roundtrip/float64', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeF64(de)
        })
}

/**
* @param {number} x 
* @returns {Promise<number>} 
*/
export async function u8 (x) {
    const out = []
    serializeU8(out, x)

    return fetch('ipc://localhost/roundtrip/u8', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeU8(de)
        })
}

/**
* @param {number} x 
* @returns {Promise<number>} 
*/
export async function s8 (x) {
    const out = []
    serializeS8(out, x)

    return fetch('ipc://localhost/roundtrip/s8', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeS8(de)
        })
}

/**
* @param {number} x 
* @returns {Promise<number>} 
*/
export async function u16 (x) {
    const out = []
    serializeU16(out, x)

    return fetch('ipc://localhost/roundtrip/u16', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeU16(de)
        })
}

/**
* @param {number} x 
* @returns {Promise<number>} 
*/
export async function s16 (x) {
    const out = []
    serializeS16(out, x)

    return fetch('ipc://localhost/roundtrip/s16', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeS16(de)
        })
}

/**
* @param {number} x 
* @returns {Promise<number>} 
*/
export async function u32 (x) {
    const out = []
    serializeU32(out, x)

    return fetch('ipc://localhost/roundtrip/u32', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeU32(de)
        })
}

/**
* @param {number} x 
* @returns {Promise<number>} 
*/
export async function s32 (x) {
    const out = []
    serializeS32(out, x)

    return fetch('ipc://localhost/roundtrip/s32', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeS32(de)
        })
}

/**
* @param {bigint} x 
* @returns {Promise<bigint>} 
*/
export async function u64 (x) {
    const out = []
    serializeU64(out, x)

    return fetch('ipc://localhost/roundtrip/u64', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeU64(de)
        })
}

/**
* @param {bigint} x 
* @returns {Promise<bigint>} 
*/
export async function s64 (x) {
    const out = []
    serializeS64(out, x)

    return fetch('ipc://localhost/roundtrip/s64', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeS64(de)
        })
}

/**
* @param {bigint} x 
* @returns {Promise<bigint>} 
*/
export async function u128 (x) {
    const out = []
    serializeU128(out, x)

    return fetch('ipc://localhost/roundtrip/u128', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeU128(de)
        })
}

/**
* @param {bigint} x 
* @returns {Promise<bigint>} 
*/
export async function s128 (x) {
    const out = []
    serializeS128(out, x)

    return fetch('ipc://localhost/roundtrip/s128', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeS128(de)
        })
}

/**
* @param {Uint8Array[]} x 
* @returns {Promise<Uint8Array[]>} 
*/
export async function listU8 (x) {
    const out = []
    serializeBytes(out, x)

    return fetch('ipc://localhost/roundtrip/list_u8', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeBytes(de)
        })
}

/**
* @param {Uint16Array[]} x 
* @returns {Promise<Uint16Array[]>} 
*/
export async function listU16 (x) {
    const out = []
    serializeList(out, (out, v) => serializeU16(out, v), x)

    return fetch('ipc://localhost/roundtrip/list_u16', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeU16(de))
        })
}

/**
* @param {Uint32Array[]} x 
* @returns {Promise<Uint32Array[]>} 
*/
export async function listU32 (x) {
    const out = []
    serializeList(out, (out, v) => serializeU32(out, v), x)

    return fetch('ipc://localhost/roundtrip/list_u32', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeU32(de))
        })
}

/**
* @param {BigUint64Array[]} x 
* @returns {Promise<BigUint64Array[]>} 
*/
export async function listU64 (x) {
    const out = []
    serializeList(out, (out, v) => serializeU64(out, v), x)

    return fetch('ipc://localhost/roundtrip/list_u64', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeU64(de))
        })
}

/**
* @param {bigint[]} x 
* @returns {Promise<bigint[]>} 
*/
export async function listU128 (x) {
    const out = []
    serializeList(out, (out, v) => serializeU128(out, v), x)

    return fetch('ipc://localhost/roundtrip/list_u128', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeU128(de))
        })
}

/**
* @param {Int8Array[]} x 
* @returns {Promise<Int8Array[]>} 
*/
export async function listS8 (x) {
    const out = []
    serializeList(out, (out, v) => serializeS8(out, v), x)

    return fetch('ipc://localhost/roundtrip/list_s8', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeS8(de))
        })
}

/**
* @param {Int16Array[]} x 
* @returns {Promise<Int16Array[]>} 
*/
export async function listS16 (x) {
    const out = []
    serializeList(out, (out, v) => serializeS16(out, v), x)

    return fetch('ipc://localhost/roundtrip/list_s16', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeS16(de))
        })
}

/**
* @param {Int32Array[]} x 
* @returns {Promise<Int32Array[]>} 
*/
export async function listS32 (x) {
    const out = []
    serializeList(out, (out, v) => serializeS32(out, v), x)

    return fetch('ipc://localhost/roundtrip/list_s32', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeS32(de))
        })
}

/**
* @param {BigInt64Array[]} x 
* @returns {Promise<BigInt64Array[]>} 
*/
export async function listS64 (x) {
    const out = []
    serializeList(out, (out, v) => serializeS64(out, v), x)

    return fetch('ipc://localhost/roundtrip/list_s64', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeS64(de))
        })
}

/**
* @param {bigint[]} x 
* @returns {Promise<bigint[]>} 
*/
export async function listS128 (x) {
    const out = []
    serializeList(out, (out, v) => serializeS128(out, v), x)

    return fetch('ipc://localhost/roundtrip/list_s128', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeS128(de))
        })
}

/**
* @param {Float32Array[]} x 
* @returns {Promise<Float32Array[]>} 
*/
export async function listFloat32 (x) {
    const out = []
    serializeList(out, (out, v) => serializeF32(out, v), x)

    return fetch('ipc://localhost/roundtrip/list_float32', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeF32(de))
        })
}

/**
* @param {Float64Array[]} x 
* @returns {Promise<Float64Array[]>} 
*/
export async function listFloat64 (x) {
    const out = []
    serializeList(out, (out, v) => serializeF64(out, v), x)

    return fetch('ipc://localhost/roundtrip/list_float64', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeF64(de))
        })
}

/**
* @param {[number, number][]} x 
* @returns {Promise<[number, number][]>} 
*/
export async function tupleList (x) {
    const out = []
    serializeList(out, (out, v) => {serializeU8(out, v[0]);serializeS8(out, v[1])}, x)

    return fetch('ipc://localhost/roundtrip/tuple_list', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => [deserializeU8(de), deserializeS8(de)])
        })
}

/**
* @param {string[]} a 
* @returns {Promise<string[]>} 
*/
export async function stringList (a) {
    const out = []
    serializeList(out, (out, v) => serializeString(out, v), a)

    return fetch('ipc://localhost/roundtrip/string_list', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeString(de))
        })
}

/**
* @param {[number, string][]} x 
* @returns {Promise<[number, string][]>} 
*/
export async function tupleStringList (x) {
    const out = []
    serializeList(out, (out, v) => {serializeU8(out, v[0]);serializeString(out, v[1])}, x)

    return fetch('ipc://localhost/roundtrip/tuple_string_list', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => [deserializeU8(de), deserializeString(de)])
        })
}

/**
* @param {SomeRecord[]} x 
* @returns {Promise<OtherRecord[]>} 
*/
export async function recordList (x) {
    const out = []
    serializeList(out, (out, v) => serializeSomeRecord(out, v), x)

    return fetch('ipc://localhost/roundtrip/record_list', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeOtherRecord(de))
        })
}

/**
* @param {AllIntegers} x 
* @returns {Promise<AllIntegers>} 
*/
export async function allIntegers (x) {
    const out = []
    serializeAllIntegers(out, x)

    return fetch('ipc://localhost/roundtrip/all_integers', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeAllIntegers(de)
        })
}

/**
* @param {AllFloats} x 
* @returns {Promise<AllFloats>} 
*/
export async function allFloats (x) {
    const out = []
    serializeAllFloats(out, x)

    return fetch('ipc://localhost/roundtrip/all_floats', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeAllFloats(de)
        })
}

/**
* @param {AllText} x 
* @returns {Promise<AllText>} 
*/
export async function allText (x) {
    const out = []
    serializeAllText(out, x)

    return fetch('ipc://localhost/roundtrip/all_text', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeAllText(de)
        })
}

/**
* @param {E1} x 
* @returns {Promise<E1>} 
*/
export async function e1 (x) {
    const out = []
    serializeE1(out, x)

    return fetch('ipc://localhost/roundtrip/e1', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeE1(de)
        })
}

/**
* @param {V1} x 
* @returns {Promise<V1>} 
*/
export async function v1 (x) {
    const out = []
    serializeV1(out, x)

    return fetch('ipc://localhost/roundtrip/v1', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeV1(de)
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
    const out = []
    serializeOption(out, (out, v) => serializeBool(out, v), a);
serializeOption(out, (out, v) => {}, b);
serializeOption(out, (out, v) => serializeU32(out, v), c);
serializeOption(out, (out, v) => serializeE1(out, v), d);
serializeOption(out, (out, v) => serializeF32(out, v), e);
serializeOption(out, (out, v) => serializeU1(out, v), f);
serializeOption(out, (out, v) => serializeOption(out, (out, v) => serializeBool(out, v), v), g)

    return fetch('ipc://localhost/roundtrip/options', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return [deserializeOption(de, (de) => deserializeBool(de)), deserializeOption(de, (de) => []), deserializeOption(de, (de) => deserializeU32(de)), deserializeOption(de, (de) => deserializeE1(de)), deserializeOption(de, (de) => deserializeF32(de)), deserializeOption(de, (de) => deserializeU1(de)), deserializeOption(de, (de) => deserializeOption(de, (de) => deserializeBool(de)))]
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
    const out = []
    serializeResult(out, (out, v) => {}, (out, v) => {}, a);
serializeResult(out, (out, v) => {}, (out, v) => serializeE1(out, v), b);
serializeResult(out, (out, v) => serializeE1(out, v), (out, v) => {}, c);
serializeResult(out, (out, v) => {}, (out, v) => {}, d);
serializeResult(out, (out, v) => serializeU32(out, v), (out, v) => serializeV1(out, v), e);
serializeResult(out, (out, v) => serializeString(out, v), (out, v) => serializeBytes(out, v), f)

    return fetch('ipc://localhost/roundtrip/results', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return [deserializeResult(de, () => {}, () => {}), deserializeResult(de, () => {}, deserializeE1(de)), deserializeResult(de, deserializeE1(de), () => {}), deserializeResult(de, [], []), deserializeResult(de, deserializeU32(de), deserializeV1(de)), deserializeResult(de, deserializeString(de), deserializeBytes(de))]
        })
}

