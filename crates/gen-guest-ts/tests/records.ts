// @ts-nocheck
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
function deserializeU32(de) {
    return de_varint(de, 32)
}
function deserializeU64(de) {
  return de_varint_big(de, 64)
}
function deserializeS32(de) {
    const n = de_varint(de, 32)

    return Number(((n >> 1) & 0xFFFFFFFF) ^ (-((n & 0b1) & 0xFFFFFFFF)))
}
function deserializeS64(de) {
  const n = de_varint_big(de, 64)

  return ((n >> 1n) & 0xFFFFFFFFFFFFFFFFn) ^ (-((n & 0b1n) & 0xFFFFFFFFFFFFFFFFn))
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
function serializeU32(out, val) {
    return ser_varint(out, 32, val)
}
function serializeU64(out, val) {
  return ser_varint_big(out, 64, BigInt(val))
}
function serializeS32(out, val) {
    ser_varint(out, 32, (val << 1) ^ (val >> 31))
}
function serializeS64(out, val) {
  val = BigInt(val)
  ser_varint_big(out, 64, (val << 1n) ^ (val >> 63n))
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
}function serializeIntTypedef(out, val) {
    serializeS32(out, val)
}function serializeTupleTypedef2(out, val) {
    {serializeIntTypedef(out, val[0])}
}

export interface Empty {  }
/**
 * A record containing two scalar fields
 * that both have the same type
*/
export interface Scalars { /**
 * The first field, named a
*/
a: number,
/**
 * The second field, named b
*/
b: number,
 }
/**
 * A record that is really just flags
 * All of the fields are bool
*/
export interface ReallyFlags { 
a: boolean,

b: boolean,

c: boolean,

d: boolean,

e: boolean,

f: boolean,

g: boolean,

h: boolean,

i: boolean,
 }

export interface Aggregates { 
a: Scalars,

b: number,

c: Empty,

d: string,

e: ReallyFlags,
 }

export type IntTypedef = number;

export type TupleTypedef2 = [IntTypedef];



export async function tupleArg (x: [string, number]) : Promise<void> {
    const out = []
    {serializeChar(out, x[0]);serializeU32(out, x[1])}

     fetch('ipc://localhost/records/tuple_arg', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function tupleResult () : Promise<[string, number]> {
    const out = []
    

    return fetch('ipc://localhost/records/tuple_result', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return [deserializeChar(de), deserializeU32(de)]
        }) as Promise<[string, number]>
}
        

export async function emptyArg (x: Empty) : Promise<void> {
    const out = []
    serializeEmpty(out, x)

     fetch('ipc://localhost/records/empty_arg', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function emptyResult () : Promise<Empty> {
    const out = []
    

    return fetch('ipc://localhost/records/empty_result', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeEmpty(de)
        }) as Promise<Empty>
}
        

export async function scalarArg (x: Scalars) : Promise<void> {
    const out = []
    serializeScalars(out, x)

     fetch('ipc://localhost/records/scalar_arg', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function scalarResult () : Promise<Scalars> {
    const out = []
    

    return fetch('ipc://localhost/records/scalar_result', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeScalars(de)
        }) as Promise<Scalars>
}
        

export async function flagsArg (x: ReallyFlags) : Promise<void> {
    const out = []
    serializeReallyFlags(out, x)

     fetch('ipc://localhost/records/flags_arg', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function flagsResult () : Promise<ReallyFlags> {
    const out = []
    

    return fetch('ipc://localhost/records/flags_result', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeReallyFlags(de)
        }) as Promise<ReallyFlags>
}
        

export async function aggregateArg (x: Aggregates) : Promise<void> {
    const out = []
    serializeAggregates(out, x)

     fetch('ipc://localhost/records/aggregate_arg', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function aggregateResult () : Promise<Aggregates> {
    const out = []
    

    return fetch('ipc://localhost/records/aggregate_result', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeAggregates(de)
        }) as Promise<Aggregates>
}
        

export async function typedefInout (e: TupleTypedef2) : Promise<number> {
    const out = []
    serializeTupleTypedef2(out, e)

    return fetch('ipc://localhost/records/typedef_inout', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeS32(de)
        }) as Promise<number>
}
        