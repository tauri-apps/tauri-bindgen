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
  let out = 0;

  for (let i = 0; i < varint_max(type); i++) {
    const val = de.pop();
    const carry = val & 0x7F;
    out |= carry << (7 * i);

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

function de_varint_big(de, type) {
  let out = 0n;

  for (let i = 0; i < varint_max(type); i++) {
    const val = de.pop();
    const carry = BigInt(val) & 0x7Fn;
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

function ser_varint_big(out, type, val) {
  let buf = []
  for (let i = 0; i < varint_max(type); i++) {
    const buffer = new ArrayBuffer(type / 8);
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
const __text_decoder = new TextDecoder('utf-8');
const __text_encoder = new TextEncoder();
function deserializeAllIntegers(de) {
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
}function deserializeDuplicatedS32(de) {
    const tag = deserializeU32(de)

    switch (tag) {
        case 0:
    return { S320: deserializeS32(de) }
case 1:
    return { S321: deserializeS32(de) }
case 2:
    return { S322: deserializeS32(de) }

        default:
            throw new Error(`unknown union case ${tag}`)
    }
}function deserializeDistinguishableNum(de) {
    const tag = deserializeU32(de)

    switch (tag) {
        case 0:
    return { F64: deserializeF64(de) }
case 1:
    return { S64: deserializeS64(de) }

        default:
            throw new Error(`unknown union case ${tag}`)
    }
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
}function serializeDuplicatedS32(out, val) {
    if (val.S320) {
    serializeU32(out, 0);
    return serializeS32(out, val.S320)
}
                if (val.S321) {
    serializeU32(out, 1);
    return serializeS32(out, val.S321)
}
                if (val.S322) {
    serializeU32(out, 2);
    return serializeS32(out, val.S322)
}
                

    throw new Error("unknown union case")
}function serializeDistinguishableNum(out, val) {
    if (val.F64) {
    serializeU32(out, 0);
    return serializeF64(out, val.F64)
}
                if (val.S64) {
    serializeU32(out, 1);
    return serializeS64(out, val.S64)
}
                

    throw new Error("unknown union case")
}
/**
 * A union of all of the integral types 
*/
export type AllIntegers = /**
 * Bool is equivalent to a 1 bit integer 
 * and is treated that way in some languages 
*/
boolean
 | 
number
 | 
number
 | 
number
 | 
bigint
 | 
number
 | 
number
 | 
number
 | 
bigint
;

export type AllFloats = 
number
 | 
number
;

export type AllText = 
string
 | 
string
;

export type DuplicatedS32 = /**
 * The first s32 
*/
number
 | /**
 * The second s32 
*/
number
 | /**
 * The third s32 
*/
number
;
/**
 * A type containing numeric types that are distinct in most languages 
*/
export type DistinguishableNum = /**
 * A Floating Point Number 
*/
number
 | /**
 * A Signed Integer 
*/
bigint
;



export async function addOneInteger (num: AllIntegers) : Promise<AllIntegers> {
    const out = []
    serializeAllIntegers(out, num)
    
    return fetch('ipc://localhost/unions/add_one_integer', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeAllIntegers(de)
        }) as Promise<AllIntegers>
}
        

export async function addOneFloat (num: AllFloats) : Promise<AllFloats> {
    const out = []
    serializeAllFloats(out, num)
    
    return fetch('ipc://localhost/unions/add_one_float', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeAllFloats(de)
        }) as Promise<AllFloats>
}
        

export async function replaceFirstChar (text: AllText, letter: string) : Promise<AllText> {
    const out = []
    serializeAllText(out, text);
serializeChar(out, letter)
    
    return fetch('ipc://localhost/unions/replace_first_char', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeAllText(de)
        }) as Promise<AllText>
}
        

export async function identifyInteger (num: AllIntegers) : Promise<number> {
    const out = []
    serializeAllIntegers(out, num)
    
    return fetch('ipc://localhost/unions/identify_integer', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeU8(de)
        }) as Promise<number>
}
        

export async function identifyFloat (num: AllFloats) : Promise<number> {
    const out = []
    serializeAllFloats(out, num)
    
    return fetch('ipc://localhost/unions/identify_float', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeU8(de)
        }) as Promise<number>
}
        

export async function identifyText (text: AllText) : Promise<number> {
    const out = []
    serializeAllText(out, text)
    
    return fetch('ipc://localhost/unions/identify_text', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeU8(de)
        }) as Promise<number>
}
        

export async function addOneDuplicated (num: DuplicatedS32) : Promise<DuplicatedS32> {
    const out = []
    serializeDuplicatedS32(out, num)
    
    return fetch('ipc://localhost/unions/add_one_duplicated', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeDuplicatedS32(de)
        }) as Promise<DuplicatedS32>
}
        

export async function identifyDuplicated (num: DuplicatedS32) : Promise<number> {
    const out = []
    serializeDuplicatedS32(out, num)
    
    return fetch('ipc://localhost/unions/identify_duplicated', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeU8(de)
        }) as Promise<number>
}
        

export async function addOneDistinguishableNum (num: DistinguishableNum) : Promise<DistinguishableNum> {
    const out = []
    serializeDistinguishableNum(out, num)
    
    return fetch('ipc://localhost/unions/add_one_distinguishable_num', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeDistinguishableNum(de)
        }) as Promise<DistinguishableNum>
}
        

export async function identifyDistinguishableNum (num: DistinguishableNum) : Promise<number> {
    const out = []
    serializeDistinguishableNum(out, num)
    
    return fetch('ipc://localhost/unions/identify_distinguishable_num', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeU8(de)
        }) as Promise<number>
}
        