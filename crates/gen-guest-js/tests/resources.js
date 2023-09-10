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


/**
* @returns {Promise<A>} 
*/
export async function constructorA () {
    const out = []
    

    return fetch('ipc://localhost/resources/constructor_a', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return A.deserialize(de)
        })
}

/**
* @returns {Promise<B>} 
*/
export async function constructorB () {
    const out = []
    

    return fetch('ipc://localhost/resources/constructor_b', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return B.deserialize(de)
        })
}


export class A {
            #id;
            /**
*/
async f1 () {
    const out = []
    serializeU32(out, this.#id);
    

    await fetch('ipc://localhost/resources::resource::a/f1', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
}
/**
* @param {number} a 
*/
async f2 (a) {
    const out = []
    serializeU32(out, this.#id);
    serializeU32(out, a)

    await fetch('ipc://localhost/resources::resource::a/f2', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
}
/**
* @param {number} a 
* @param {number} b 
*/
async f3 (a, b) {
    const out = []
    serializeU32(out, this.#id);
    serializeU32(out, a);
serializeU32(out, b)

    await fetch('ipc://localhost/resources::resource::a/f3', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
}

            static deserialize(de) {
    const self = new A();
    self.#id = deserializeU32(de);
    return self
}
        }
export class B {
            #id;
            /**
* @returns {Promise<A>} 
*/
async f1 () {
    const out = []
    serializeU32(out, this.#id);
    

    await fetch('ipc://localhost/resources::resource::b/f1', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return A.deserialize(de)
        })
}
/**
* @param {A} x 
* @returns {Promise<Result<number, _>>} 
*/
async f2 (x) {
    const out = []
    serializeU32(out, this.#id);
    x.serialize(out)

    await fetch('ipc://localhost/resources::resource::b/f2', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeResult(de, (de) => deserializeU32(de), () => {})
        })
}
/**
* @param {A[] | null} x 
* @returns {Promise<Result<A, _>>} 
*/
async f3 (x) {
    const out = []
    serializeU32(out, this.#id);
    serializeOption(out, (out, v) => serializeList(out, (out, v) => v.serialize(out), v), x)

    await fetch('ipc://localhost/resources::resource::b/f3', { method: "POST", body: Uint8Array.from(out), headers: { 'Content-Type': 'application/octet-stream' } })
        .then(r => r.arrayBuffer())
        .then(bytes => {
            const de = new Deserializer(new Uint8Array(bytes))

            return deserializeResult(de, (de) => A.deserialize(de), () => {})
        })
}

            static deserialize(de) {
    const self = new B();
    self.#id = deserializeU32(de);
    return self
}
        }