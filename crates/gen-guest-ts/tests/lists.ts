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
function deserializeBytes(de) {
    const sz = deserializeU64(de);

    let bytes = de.try_take_n(Number(sz));

    return bytes;
}
function deserializeList(de, inner) {
    const len = deserializeU64(de);

    let out = [];

    for (let i = 0; i < len; i++) {
        out.push(inner(de));   
    }

    return out;
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
function serializeBytes(out, val) {
    serializeU64(out, val.length);
    out.push(...val)
}
function serializeList(out, inner, val) {
    serializeU64(out, val.length)
    for (const el of val) {
        inner(out, el)
    }
}
const __text_decoder = new TextDecoder('utf-8');
const __text_encoder = new TextEncoder();
function deserializeOtherRecord(de) {
    return {
        a1: deserializeU32(de),
a2: deserializeU64(de),
a3: deserializeS32(de),
a4: deserializeS64(de),
b: deserializeString(de),
c: deserializeBytes(de)
    }
}function deserializeSomeRecord(de) {
    return {
        x: deserializeString(de),
y: deserializeOtherRecord(de),
z: deserializeList(de, (de) => deserializeOtherRecord(de)),
c1: deserializeU32(de),
c2: deserializeU64(de),
c3: deserializeS32(de),
c4: deserializeS64(de)
    }
}function deserializeOtherVariant(de) {
    const tag = deserializeU32(de)

    switch (tag) {
        case 0:
    return { A: null }
case 1:
    return { B: deserializeU32(de) }
case 2:
    return { C: deserializeString(de) }

        default:
            throw new Error(`unknown variant case ${tag}`)
    }
}function deserializeLoadStoreAllSizes(de) {
    return deserializeList(de, (de) => [deserializeString(de), deserializeU8(de), deserializeS8(de), deserializeU16(de), deserializeS16(de), deserializeU32(de), deserializeS32(de), deserializeU64(de), deserializeS64(de), deserializeF32(de), deserializeF64(de), deserializeChar(de)])
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
}function serializeOtherVariant(out, val) {
    if (val.A) {
    serializeU32(out, 0);
    return 
}
if (val.B) {
    serializeU32(out, 1);
    return serializeU32(out, val.B)
}
if (val.C) {
    serializeU32(out, 2);
    return serializeString(out, val.C)
}


    throw new Error("unknown variant case")
}function serializeSomeVariant(out, val) {
    if (val.A) {
    serializeU32(out, 0);
    return serializeString(out, val.A)
}
if (val.B) {
    serializeU32(out, 1);
    return 
}
if (val.C) {
    serializeU32(out, 2);
    return serializeU32(out, val.C)
}
if (val.D) {
    serializeU32(out, 3);
    return serializeList(out, (out, v) => serializeOtherVariant(out, v), val.D)
}


    throw new Error("unknown variant case")
}function serializeLoadStoreAllSizes(out, val) {
    serializeList(out, (out, v) => {serializeString(out, v[0]);serializeU8(out, v[1]);serializeS8(out, v[2]);serializeU16(out, v[3]);serializeS16(out, v[4]);serializeU32(out, v[5]);serializeS32(out, v[6]);serializeU64(out, v[7]);serializeS64(out, v[8]);serializeF32(out, v[9]);serializeF64(out, v[10]);serializeChar(out, v[11])}, val)
}

export interface OtherRecord { 
a1: number,

a2: bigint,

a3: number,

a4: bigint,

b: string,

c: Uint8Array[],
 }

export interface SomeRecord { 
x: string,

y: OtherRecord,

z: OtherRecord[],

c1: number,

c2: bigint,

c3: number,

c4: bigint,
 }

export interface OtherVariantA { tag: 0 }

export interface OtherVariantB { tag: 1, value: number }

export interface OtherVariantC { tag: 2, value: string }


export type OtherVariant = 
OtherVariantA | 
OtherVariantB | 
OtherVariantC

export interface SomeVariantA { tag: 0, value: string }

export interface SomeVariantB { tag: 1 }

export interface SomeVariantC { tag: 2, value: number }

export interface SomeVariantD { tag: 3, value: OtherVariant[] }


export type SomeVariant = 
SomeVariantA | 
SomeVariantB | 
SomeVariantC | 
SomeVariantD

export type LoadStoreAllSizes = [string, number, number, number, number, number, number, bigint, bigint, number, number, string][];



export async function listU8Param (x: Uint8Array[]) : Promise<void> {
    const out = []
    serializeBytes(out, x)
    
     fetch('ipc://localhost/lists/list_u8_param', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function listU16Param (x: Uint16Array[]) : Promise<void> {
    const out = []
    serializeList(out, (out, v) => serializeU16(out, v), x)
    
     fetch('ipc://localhost/lists/list_u16_param', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function listU32Param (x: Uint32Array[]) : Promise<void> {
    const out = []
    serializeList(out, (out, v) => serializeU32(out, v), x)
    
     fetch('ipc://localhost/lists/list_u32_param', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function listU64Param (x: BigUint64Array[]) : Promise<void> {
    const out = []
    serializeList(out, (out, v) => serializeU64(out, v), x)
    
     fetch('ipc://localhost/lists/list_u64_param', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function listS8Param (x: Int8Array[]) : Promise<void> {
    const out = []
    serializeList(out, (out, v) => serializeS8(out, v), x)
    
     fetch('ipc://localhost/lists/list_s8_param', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function listS16Param (x: Int16Array[]) : Promise<void> {
    const out = []
    serializeList(out, (out, v) => serializeS16(out, v), x)
    
     fetch('ipc://localhost/lists/list_s16_param', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function listS32Param (x: Int32Array[]) : Promise<void> {
    const out = []
    serializeList(out, (out, v) => serializeS32(out, v), x)
    
     fetch('ipc://localhost/lists/list_s32_param', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function listS64Param (x: BigInt64Array[]) : Promise<void> {
    const out = []
    serializeList(out, (out, v) => serializeS64(out, v), x)
    
     fetch('ipc://localhost/lists/list_s64_param', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function listFloat32Param (x: Float32Array[]) : Promise<void> {
    const out = []
    serializeList(out, (out, v) => serializeF32(out, v), x)
    
     fetch('ipc://localhost/lists/list_float32_param', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function listFloat64Param (x: Float64Array[]) : Promise<void> {
    const out = []
    serializeList(out, (out, v) => serializeF64(out, v), x)
    
     fetch('ipc://localhost/lists/list_float64_param', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function listU8Ret () : Promise<Uint8Array[]> {
    const out = []
    
    
    return fetch('ipc://localhost/lists/list_u8_ret', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeBytes(de)
        }) as Promise<Uint8Array[]>
}
        

export async function listU16Ret () : Promise<Uint16Array[]> {
    const out = []
    
    
    return fetch('ipc://localhost/lists/list_u16_ret', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeU16(de))
        }) as Promise<Uint16Array[]>
}
        

export async function listU32Ret () : Promise<Uint32Array[]> {
    const out = []
    
    
    return fetch('ipc://localhost/lists/list_u32_ret', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeU32(de))
        }) as Promise<Uint32Array[]>
}
        

export async function listU64Ret () : Promise<BigUint64Array[]> {
    const out = []
    
    
    return fetch('ipc://localhost/lists/list_u64_ret', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeU64(de))
        }) as Promise<BigUint64Array[]>
}
        

export async function listS8Ret () : Promise<Int8Array[]> {
    const out = []
    
    
    return fetch('ipc://localhost/lists/list_s8_ret', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeS8(de))
        }) as Promise<Int8Array[]>
}
        

export async function listS16Ret () : Promise<Int16Array[]> {
    const out = []
    
    
    return fetch('ipc://localhost/lists/list_s16_ret', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeS16(de))
        }) as Promise<Int16Array[]>
}
        

export async function listS32Ret () : Promise<Int32Array[]> {
    const out = []
    
    
    return fetch('ipc://localhost/lists/list_s32_ret', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeS32(de))
        }) as Promise<Int32Array[]>
}
        

export async function listS64Ret () : Promise<BigInt64Array[]> {
    const out = []
    
    
    return fetch('ipc://localhost/lists/list_s64_ret', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeS64(de))
        }) as Promise<BigInt64Array[]>
}
        

export async function listFloat32Ret () : Promise<Float32Array[]> {
    const out = []
    
    
    return fetch('ipc://localhost/lists/list_float32_ret', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeF32(de))
        }) as Promise<Float32Array[]>
}
        

export async function listFloat64Ret () : Promise<Float64Array[]> {
    const out = []
    
    
    return fetch('ipc://localhost/lists/list_float64_ret', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeF64(de))
        }) as Promise<Float64Array[]>
}
        

export async function tupleList (x: [number, number][]) : Promise<[bigint, number][]> {
    const out = []
    serializeList(out, (out, v) => {serializeU8(out, v[0]);serializeS8(out, v[1])}, x)
    
    return fetch('ipc://localhost/lists/tuple_list', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => [deserializeS64(de), deserializeU32(de)])
        }) as Promise<[bigint, number][]>
}
        

export async function stringListArg (a: string[]) : Promise<void> {
    const out = []
    serializeList(out, (out, v) => serializeString(out, v), a)
    
     fetch('ipc://localhost/lists/string_list_arg', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function stringListRet () : Promise<string[]> {
    const out = []
    
    
    return fetch('ipc://localhost/lists/string_list_ret', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeString(de))
        }) as Promise<string[]>
}
        

export async function tupleStringList (x: [number, string][]) : Promise<[string, number][]> {
    const out = []
    serializeList(out, (out, v) => {serializeU8(out, v[0]);serializeString(out, v[1])}, x)
    
    return fetch('ipc://localhost/lists/tuple_string_list', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => [deserializeString(de), deserializeU8(de)])
        }) as Promise<[string, number][]>
}
        

export async function stringList (x: string[]) : Promise<string[]> {
    const out = []
    serializeList(out, (out, v) => serializeString(out, v), x)
    
    return fetch('ipc://localhost/lists/string_list', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeString(de))
        }) as Promise<string[]>
}
        

export async function recordList (x: SomeRecord[]) : Promise<OtherRecord[]> {
    const out = []
    serializeList(out, (out, v) => serializeSomeRecord(out, v), x)
    
    return fetch('ipc://localhost/lists/record_list', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeOtherRecord(de))
        }) as Promise<OtherRecord[]>
}
        

export async function recordListReverse (x: OtherRecord[]) : Promise<SomeRecord[]> {
    const out = []
    serializeList(out, (out, v) => serializeOtherRecord(out, v), x)
    
    return fetch('ipc://localhost/lists/record_list_reverse', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeSomeRecord(de))
        }) as Promise<SomeRecord[]>
}
        

export async function variantList (x: SomeVariant[]) : Promise<OtherVariant[]> {
    const out = []
    serializeList(out, (out, v) => serializeSomeVariant(out, v), x)
    
    return fetch('ipc://localhost/lists/variant_list', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeOtherVariant(de))
        }) as Promise<OtherVariant[]>
}
        

export async function loadStoreEverything (a: LoadStoreAllSizes) : Promise<LoadStoreAllSizes> {
    const out = []
    serializeLoadStoreAllSizes(out, a)
    
    return fetch('ipc://localhost/lists/load_store_everything', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeLoadStoreAllSizes(de)
        }) as Promise<LoadStoreAllSizes>
}
        