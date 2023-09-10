// @ts-nocheck
export type Result<T, E> = { tag: 'ok', val: T } | { tag: 'err', val: E };
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
            return { tag: 'ok', val: ok(de) }
        case 1: 
            return { tag: 'err', val: err(de) }
        default:
            throw new Error(`Deserialize bad result ${tag}`)
    }
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
function serializeU32(out, val) {
    return ser_varint(out, 32, val)
}
function serializeU64(out, val) {
  return ser_varint_big(out, 64, BigInt(val))
}
function serializeS8(out, val) {
    out.push(val)
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
const __text_decoder = new TextDecoder('utf-8');
const __text_encoder = new TextEncoder();
function deserializeE1(de) {
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
}function deserializeEmpty(de) {
    return {
        
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
}function deserializeCasts1(de) {
    const tag = deserializeU32(de)

    switch (tag) {
        case 0:
    return { A: deserializeS32(de) }
case 1:
    return { B: deserializeF32(de) }

        default:
            throw new Error(`unknown variant case ${tag}`)
    }
}function deserializeCasts2(de) {
    const tag = deserializeU32(de)

    switch (tag) {
        case 0:
    return { A: deserializeF64(de) }
case 1:
    return { B: deserializeF32(de) }

        default:
            throw new Error(`unknown variant case ${tag}`)
    }
}function deserializeCasts3(de) {
    const tag = deserializeU32(de)

    switch (tag) {
        case 0:
    return { A: deserializeF64(de) }
case 1:
    return { B: deserializeU64(de) }

        default:
            throw new Error(`unknown variant case ${tag}`)
    }
}function deserializeCasts4(de) {
    const tag = deserializeU32(de)

    switch (tag) {
        case 0:
    return { A: deserializeU32(de) }
case 1:
    return { B: deserializeS64(de) }

        default:
            throw new Error(`unknown variant case ${tag}`)
    }
}function deserializeCasts5(de) {
    const tag = deserializeU32(de)

    switch (tag) {
        case 0:
    return { A: deserializeF32(de) }
case 1:
    return { B: deserializeS64(de) }

        default:
            throw new Error(`unknown variant case ${tag}`)
    }
}function deserializeCasts6(de) {
    const tag = deserializeU32(de)

    switch (tag) {
        case 0:
    return { A: [deserializeF32(de), deserializeU32(de)] }
case 1:
    return { B: [deserializeU32(de), deserializeU32(de)] }

        default:
            throw new Error(`unknown variant case ${tag}`)
    }
}function deserializeMyErrno(de) {
    const tag = deserializeU32(de)

    switch (tag) {
        case 0:
    return "Bad1"
case 1:
    return "Bad2"

        default:
            throw new Error(`unknown enum case ${tag}`)
    }
}function deserializeIsClone(de) {
    return {
        v1: deserializeV1(de)
    }
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
}function serializeEmpty(out, val) {
    
}function serializeV1(out, val) {
    if (val.A) {
    serializeU32(out, 0);
    
    return
}
if (val.B) {
    serializeU32(out, 1);
    serializeU1(out, val.B)
    return
}
if (val.C) {
    serializeU32(out, 2);
    serializeE1(out, val.C)
    return
}
if (val.D) {
    serializeU32(out, 3);
    serializeString(out, val.D)
    return
}
if (val.E) {
    serializeU32(out, 4);
    serializeEmpty(out, val.E)
    return
}
if (val.F) {
    serializeU32(out, 5);
    
    return
}
if (val.G) {
    serializeU32(out, 6);
    serializeU32(out, val.G)
    return
}


    throw new Error("unknown variant case")
}function serializeCasts1(out, val) {
    if (val.A) {
    serializeU32(out, 0);
    serializeS32(out, val.A)
    return
}
if (val.B) {
    serializeU32(out, 1);
    serializeF32(out, val.B)
    return
}


    throw new Error("unknown variant case")
}function serializeCasts2(out, val) {
    if (val.A) {
    serializeU32(out, 0);
    serializeF64(out, val.A)
    return
}
if (val.B) {
    serializeU32(out, 1);
    serializeF32(out, val.B)
    return
}


    throw new Error("unknown variant case")
}function serializeCasts3(out, val) {
    if (val.A) {
    serializeU32(out, 0);
    serializeF64(out, val.A)
    return
}
if (val.B) {
    serializeU32(out, 1);
    serializeU64(out, val.B)
    return
}


    throw new Error("unknown variant case")
}function serializeCasts4(out, val) {
    if (val.A) {
    serializeU32(out, 0);
    serializeU32(out, val.A)
    return
}
if (val.B) {
    serializeU32(out, 1);
    serializeS64(out, val.B)
    return
}


    throw new Error("unknown variant case")
}function serializeCasts5(out, val) {
    if (val.A) {
    serializeU32(out, 0);
    serializeF32(out, val.A)
    return
}
if (val.B) {
    serializeU32(out, 1);
    serializeS64(out, val.B)
    return
}


    throw new Error("unknown variant case")
}function serializeCasts6(out, val) {
    if (val.A) {
    serializeU32(out, 0);
    {serializeF32(out, val.A[0]);serializeU32(out, val.A[1])}
    return
}
if (val.B) {
    serializeU32(out, 1);
    {serializeU32(out, val.B[0]);serializeU32(out, val.B[1])}
    return
}


    throw new Error("unknown variant case")
}function serializeIsClone(out, val) {
    serializeV1(out, val.v1)
}

export enum E1 { 
A,
 }

export type U1 = 
number
 | 
number
;

export interface Empty {  }

export interface V1A { tag: 0 }

export interface V1B { tag: 1, value: U1 }

export interface V1C { tag: 2, value: E1 }

export interface V1D { tag: 3, value: string }

export interface V1E { tag: 4, value: Empty }

export interface V1F { tag: 5 }

export interface V1G { tag: 6, value: number }


export type V1 = 
V1A | 
V1B | 
V1C | 
V1D | 
V1E | 
V1F | 
V1G

export interface Casts1A { tag: 0, value: number }

export interface Casts1B { tag: 1, value: number }


export type Casts1 = 
Casts1A | 
Casts1B

export interface Casts2A { tag: 0, value: number }

export interface Casts2B { tag: 1, value: number }


export type Casts2 = 
Casts2A | 
Casts2B

export interface Casts3A { tag: 0, value: number }

export interface Casts3B { tag: 1, value: bigint }


export type Casts3 = 
Casts3A | 
Casts3B

export interface Casts4A { tag: 0, value: number }

export interface Casts4B { tag: 1, value: bigint }


export type Casts4 = 
Casts4A | 
Casts4B

export interface Casts5A { tag: 0, value: number }

export interface Casts5B { tag: 1, value: bigint }


export type Casts5 = 
Casts5A | 
Casts5B

export interface Casts6A { tag: 0, value: [number, number] }

export interface Casts6B { tag: 1, value: [number, number] }


export type Casts6 = 
Casts6A | 
Casts6B

export enum MyErrno { 
Bad1,

Bad2,
 }

export interface IsClone { 
v1: V1,
 }



export async function e1Arg (x: E1) : Promise<void> {
    const out = []
    serializeE1(out, x)

     fetch('ipc://localhost/variants/e1_arg', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function e1Result () : Promise<E1> {
    const out = []
    

    return fetch('ipc://localhost/variants/e1_result', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeE1(de)
        }) as Promise<E1>
}
        

export async function u1Arg (x: U1) : Promise<void> {
    const out = []
    serializeU1(out, x)

     fetch('ipc://localhost/variants/u1_arg', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function u1Result () : Promise<U1> {
    const out = []
    

    return fetch('ipc://localhost/variants/u1_result', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeU1(de)
        }) as Promise<U1>
}
        

export async function v1Arg (x: V1) : Promise<void> {
    const out = []
    serializeV1(out, x)

     fetch('ipc://localhost/variants/v1_arg', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function v1Result () : Promise<V1> {
    const out = []
    

    return fetch('ipc://localhost/variants/v1_result', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeV1(de)
        }) as Promise<V1>
}
        

export async function boolArg (x: boolean) : Promise<void> {
    const out = []
    serializeBool(out, x)

     fetch('ipc://localhost/variants/bool_arg', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function boolResult () : Promise<boolean> {
    const out = []
    

    return fetch('ipc://localhost/variants/bool_result', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeBool(de)
        }) as Promise<boolean>
}
        

export async function optionArg (a: boolean | null, b: [] | null, c: number | null, d: E1 | null, e: number | null, f: U1 | null, g: boolean | null | null) : Promise<void> {
    const out = []
    serializeOption(out, (out, v) => serializeBool(out, v), a);
serializeOption(out, (out, v) => {}, b);
serializeOption(out, (out, v) => serializeU32(out, v), c);
serializeOption(out, (out, v) => serializeE1(out, v), d);
serializeOption(out, (out, v) => serializeF32(out, v), e);
serializeOption(out, (out, v) => serializeU1(out, v), f);
serializeOption(out, (out, v) => serializeOption(out, (out, v) => serializeBool(out, v), v), g)

     fetch('ipc://localhost/variants/option_arg', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function optionResult () : Promise<[boolean | null, [] | null, number | null, E1 | null, number | null, U1 | null, boolean | null | null]> {
    const out = []
    

    return fetch('ipc://localhost/variants/option_result', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return [deserializeOption(de, (de) => deserializeBool(de)), deserializeOption(de, (de) => []), deserializeOption(de, (de) => deserializeU32(de)), deserializeOption(de, (de) => deserializeE1(de)), deserializeOption(de, (de) => deserializeF32(de)), deserializeOption(de, (de) => deserializeU1(de)), deserializeOption(de, (de) => deserializeOption(de, (de) => deserializeBool(de)))]
        }) as Promise<[boolean | null, [] | null, number | null, E1 | null, number | null, U1 | null, boolean | null | null]>
}
        

export async function casts (a: Casts1, b: Casts2, c: Casts3, d: Casts4, e: Casts5, f: Casts6) : Promise<[Casts1, Casts2, Casts3, Casts4, Casts5, Casts6]> {
    const out = []
    serializeCasts1(out, a);
serializeCasts2(out, b);
serializeCasts3(out, c);
serializeCasts4(out, d);
serializeCasts5(out, e);
serializeCasts6(out, f)

    return fetch('ipc://localhost/variants/casts', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return [deserializeCasts1(de), deserializeCasts2(de), deserializeCasts3(de), deserializeCasts4(de), deserializeCasts5(de), deserializeCasts6(de)]
        }) as Promise<[Casts1, Casts2, Casts3, Casts4, Casts5, Casts6]>
}
        

export async function resultArg (a: Result<null, null>, b: Result<null, E1>, c: Result<E1, null>, d: Result<[], []>, e: Result<number, V1>, f: Result<string, Uint8Array[]>) : Promise<void> {
    const out = []
    serializeResult(out, (out, v) => {}, (out, v) => {}, a);
serializeResult(out, (out, v) => {}, (out, v) => serializeE1(out, v), b);
serializeResult(out, (out, v) => serializeE1(out, v), (out, v) => {}, c);
serializeResult(out, (out, v) => {}, (out, v) => {}, d);
serializeResult(out, (out, v) => serializeU32(out, v), (out, v) => serializeV1(out, v), e);
serializeResult(out, (out, v) => serializeString(out, v), (out, v) => serializeBytes(out, v), f)

     fetch('ipc://localhost/variants/result_arg', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function resultResult () : Promise<[Result<null, null>, Result<null, E1>, Result<E1, null>, Result<[], []>, Result<number, V1>, Result<string, Uint8Array[]>]> {
    const out = []
    

    return fetch('ipc://localhost/variants/result_result', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return [deserializeResult(de, () => {}, () => {}), deserializeResult(de, () => {}, (de) => deserializeE1(de)), deserializeResult(de, (de) => deserializeE1(de), () => {}), deserializeResult(de, (de) => [], (de) => []), deserializeResult(de, (de) => deserializeU32(de), (de) => deserializeV1(de)), deserializeResult(de, (de) => deserializeString(de), (de) => deserializeBytes(de))]
        }) as Promise<[Result<null, null>, Result<null, E1>, Result<E1, null>, Result<[], []>, Result<number, V1>, Result<string, Uint8Array[]>]>
}
        

export async function returnResultSugar () : Promise<Result<number, MyErrno>> {
    const out = []
    

    return fetch('ipc://localhost/variants/return_result_sugar', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeResult(de, (de) => deserializeS32(de), (de) => deserializeMyErrno(de))
        }) as Promise<Result<number, MyErrno>>
}
        

export async function returnResultSugar2 () : Promise<Result<null, MyErrno>> {
    const out = []
    

    return fetch('ipc://localhost/variants/return_result_sugar2', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeResult(de, () => {}, (de) => deserializeMyErrno(de))
        }) as Promise<Result<null, MyErrno>>
}
        

export async function returnResultSugar3 () : Promise<Result<MyErrno, MyErrno>> {
    const out = []
    

    return fetch('ipc://localhost/variants/return_result_sugar3', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeResult(de, (de) => deserializeMyErrno(de), (de) => deserializeMyErrno(de))
        }) as Promise<Result<MyErrno, MyErrno>>
}
        

export async function returnResultSugar4 () : Promise<Result<[number, number], MyErrno>> {
    const out = []
    

    return fetch('ipc://localhost/variants/return_result_sugar4', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeResult(de, (de) => [deserializeS32(de), deserializeU32(de)], (de) => deserializeMyErrno(de))
        }) as Promise<Result<[number, number], MyErrno>>
}
        

export async function returnOptionSugar () : Promise<number | null> {
    const out = []
    

    return fetch('ipc://localhost/variants/return_option_sugar', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeOption(de, (de) => deserializeS32(de))
        }) as Promise<number | null>
}
        

export async function returnOptionSugar2 () : Promise<MyErrno | null> {
    const out = []
    

    return fetch('ipc://localhost/variants/return_option_sugar2', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeOption(de, (de) => deserializeMyErrno(de))
        }) as Promise<MyErrno | null>
}
        

export async function resultSimple () : Promise<Result<number, number>> {
    const out = []
    

    return fetch('ipc://localhost/variants/result_simple', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeResult(de, (de) => deserializeU32(de), (de) => deserializeS32(de))
        }) as Promise<Result<number, number>>
}
        

export async function isCloneArg (a: IsClone) : Promise<void> {
    const out = []
    serializeIsClone(out, a)

     fetch('ipc://localhost/variants/is_clone_arg', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function isCloneReturn () : Promise<IsClone> {
    const out = []
    

    return fetch('ipc://localhost/variants/is_clone_return', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeIsClone(de)
        }) as Promise<IsClone>
}
        

export async function returnNamedOption () : Promise<number | null> {
    const out = []
    

    return fetch('ipc://localhost/variants/return_named_option', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeOption(de, (de) => deserializeU8(de))
        }) as Promise<number | null>
}
        

export async function returnNamedResult () : Promise<Result<number, MyErrno>> {
    const out = []
    

    return fetch('ipc://localhost/variants/return_named_result', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeResult(de, (de) => deserializeU8(de), (de) => deserializeMyErrno(de))
        }) as Promise<Result<number, MyErrno>>
}
        