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
function deserializeU32(de) {
    return de_varint(de, 32)
}
function deserializeU64(de) {
  return de_varint_big(de, 64)
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
function serializeU32(out, val) {
    return ser_varint(out, 32, val)
}
function serializeU64(out, val) {
  return ser_varint_big(out, 64, BigInt(val))
}
function serializeList(out, inner, val) {
    serializeU64(out, val.length)
    for (const el of val) {
        inner(out, el)
    }
}




export async function simpleList1 (l: Uint32Array[]) : Promise<void> {
    const out = []
    serializeList(out, (out, v) => serializeU32(out, v), l)

     fetch('ipc://localhost/simple_lists/simple_list1', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function simpleList2 () : Promise<Uint32Array[]> {
    const out = []
    

    return fetch('ipc://localhost/simple_lists/simple_list2', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeU32(de))
        }) as Promise<Uint32Array[]>
}
        

export async function simpleList3 (a: Uint32Array[], b: Uint32Array[]) : Promise<[Uint32Array[], Uint32Array[]]> {
    const out = []
    serializeList(out, (out, v) => serializeU32(out, v), a);
serializeList(out, (out, v) => serializeU32(out, v), b)

    return fetch('ipc://localhost/simple_lists/simple_list3', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return [deserializeList(de, (de) => deserializeU32(de)), deserializeList(de, (de) => deserializeU32(de))]
        }) as Promise<[Uint32Array[], Uint32Array[]]>
}
        

export async function simpleList4 (l: Uint32Array[][]) : Promise<Uint32Array[][]> {
    const out = []
    serializeList(out, (out, v) => serializeList(out, (out, v) => serializeU32(out, v), v), l)

    return fetch('ipc://localhost/simple_lists/simple_list4', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeList(de, (de) => deserializeList(de, (de) => deserializeU32(de)))
        }) as Promise<Uint32Array[][]>
}
        