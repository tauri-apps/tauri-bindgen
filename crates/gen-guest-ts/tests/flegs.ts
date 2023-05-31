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
function deserializeFlag1(de) {
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
}

export enum Flag1 { 
B0 = 2,
 }

export enum Flag2 { 
B0 = 2,

B1 = 4,
 }

export enum Flag4 { 
B0 = 2,

B1 = 4,

B2 = 8,

B3 = 16,
 }

export enum Flag8 { 
B0 = 2,

B1 = 4,

B2 = 8,

B3 = 16,

B4 = 32,

B5 = 64,

B6 = 128,

B7 = 256,
 }

export enum Flag16 { 
B0 = 2,

B1 = 4,

B2 = 8,

B3 = 16,

B4 = 32,

B5 = 64,

B6 = 128,

B7 = 256,

B8 = 512,

B9 = 1024,

B10 = 2048,

B11 = 4096,

B12 = 8192,

B13 = 16384,

B14 = 32768,

B15 = 65536,
 }

export enum Flag32 { 
B0 = 2,

B1 = 4,

B2 = 8,

B3 = 16,

B4 = 32,

B5 = 64,

B6 = 128,

B7 = 256,

B8 = 512,

B9 = 1024,

B10 = 2048,

B11 = 4096,

B12 = 8192,

B13 = 16384,

B14 = 32768,

B15 = 65536,

B16 = 131072,

B17 = 262144,

B18 = 524288,

B19 = 1048576,

B20 = 2097152,

B21 = 4194304,

B22 = 8388608,

B23 = 16777216,

B24 = 33554432,

B25 = 67108864,

B26 = 134217728,

B27 = 268435456,

B28 = 536870912,

B29 = 1073741824,

B30 = 2147483648,

B31 = 4294967296,
 }

export enum Flag64 { 
B0 = 2,

B1 = 4,

B2 = 8,

B3 = 16,

B4 = 32,

B5 = 64,

B6 = 128,

B7 = 256,

B8 = 512,

B9 = 1024,

B10 = 2048,

B11 = 4096,

B12 = 8192,

B13 = 16384,

B14 = 32768,

B15 = 65536,

B16 = 131072,

B17 = 262144,

B18 = 524288,

B19 = 1048576,

B20 = 2097152,

B21 = 4194304,

B22 = 8388608,

B23 = 16777216,

B24 = 33554432,

B25 = 67108864,

B26 = 134217728,

B27 = 268435456,

B28 = 536870912,

B29 = 1073741824,

B30 = 2147483648,

B31 = 4294967296,

B32 = 8589934592,

B33 = 17179869184,

B34 = 34359738368,

B35 = 68719476736,

B36 = 137438953472,

B37 = 274877906944,

B38 = 549755813888,

B39 = 1099511627776,

B40 = 2199023255552,

B41 = 4398046511104,

B42 = 8796093022208,

B43 = 17592186044416,

B44 = 35184372088832,

B45 = 70368744177664,

B46 = 140737488355328,

B47 = 281474976710656,

B48 = 562949953421312,

B49 = 1125899906842624,

B50 = 2251799813685248,

B51 = 4503599627370496,

B52 = 9007199254740992,

B53 = 18014398509481984,

B54 = 36028797018963968,

B55 = 72057594037927936,

B56 = 144115188075855872,

B57 = 288230376151711744,

B58 = 576460752303423488,

B59 = 1152921504606846976,

B60 = 2305843009213693952,

B61 = 4611686018427387904,

B62 = 9223372036854775808,

B63 = 0,
 }



export async function roundtripFlag1 (x: Flag1) : Promise<Flag1> {
    const out = []
    serializeFlag1(out, x)
    
    return fetch('ipc://localhost/flegs/roundtrip_flag1', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeFlag1(de)
        }) as Promise<Flag1>
}
        

export async function roundtripFlag2 (x: Flag2) : Promise<Flag2> {
    const out = []
    serializeFlag2(out, x)
    
    return fetch('ipc://localhost/flegs/roundtrip_flag2', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeFlag2(de)
        }) as Promise<Flag2>
}
        

export async function roundtripFlag4 (x: Flag4) : Promise<Flag4> {
    const out = []
    serializeFlag4(out, x)
    
    return fetch('ipc://localhost/flegs/roundtrip_flag4', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeFlag4(de)
        }) as Promise<Flag4>
}
        

export async function roundtripFlag8 (x: Flag8) : Promise<Flag8> {
    const out = []
    serializeFlag8(out, x)
    
    return fetch('ipc://localhost/flegs/roundtrip_flag8', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeFlag8(de)
        }) as Promise<Flag8>
}
        

export async function roundtripFlag16 (x: Flag16) : Promise<Flag16> {
    const out = []
    serializeFlag16(out, x)
    
    return fetch('ipc://localhost/flegs/roundtrip_flag16', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeFlag16(de)
        }) as Promise<Flag16>
}
        

export async function roundtripFlag32 (x: Flag32) : Promise<Flag32> {
    const out = []
    serializeFlag32(out, x)
    
    return fetch('ipc://localhost/flegs/roundtrip_flag32', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeFlag32(de)
        }) as Promise<Flag32>
}
        

export async function roundtripFlag64 (x: Flag64) : Promise<Flag64> {
    const out = []
    serializeFlag64(out, x)
    
    return fetch('ipc://localhost/flegs/roundtrip_flag64', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeFlag64(de)
        }) as Promise<Flag64>
}
        