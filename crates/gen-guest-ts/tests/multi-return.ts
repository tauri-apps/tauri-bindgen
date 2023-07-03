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
function deserializeU32(de) {
    return de_varint(de, 32)
}
function deserializeF32(de) {
    const bytes = de.try_take_n(4);

    const buf = new ArrayBuffer(4);
    const view = new DataView(buf);

    bytes.forEach((v, i) => view.setUint8(i, v));

    return view.getFloat32(0, true);
}




export async function mra () : Promise<void> {
    const out = []
    

     fetch('ipc://localhost/multi_return/mra', { method: "POST", body: Uint8Array.from(out) }) 
}
        

export async function mrb () : Promise<void> {
    const out = []
    

    return fetch('ipc://localhost/multi_return/mrb', { method: "POST", body: Uint8Array.from(out) }) as Promise<void>
}
        

export async function mrc () : Promise<number> {
    const out = []
    

    return fetch('ipc://localhost/multi_return/mrc', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeU32(de)
        }) as Promise<number>
}
        

export async function mrd () : Promise<number> {
    const out = []
    

    return fetch('ipc://localhost/multi_return/mrd', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeU32(de)
        }) as Promise<number>
}
        

export async function mre () : Promise<[number, number]> {
    const out = []
    

    return fetch('ipc://localhost/multi_return/mre', { method: "POST", body: Uint8Array.from(out) })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(Uint8Array.from(bytes))

            return [deserializeU32(de), deserializeF32(de)]
        }) as Promise<[number, number]>
}
        