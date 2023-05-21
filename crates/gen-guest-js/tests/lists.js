export class Deserializer {
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
        function try_take_varint(de, type) {
            let out = 0n;

            for (let i = 0; i < varint_max(type); i++) {
                const val = de.pop();
                const carry = BigInt(val & 0x7F);
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
            return try_take_varint(de, 16)
        }
        function deserializeU32(de) {
            return try_take_varint(de, 32)
        }
        function deserializeU64(de) {
            return try_take_varint(de, 64)
        }
        function deserializeI8(de) {
            return de.pop()
        }
        function deserializeI16(de) {
            const n = try_take_varint(de, 16)

            return Number(((n >> 1n) & 0xFFFFn) ^ (-((n & 0b1n) & 0xFFFFn)))
        }
        function deserializeI32(de) {
            const n = try_take_varint(de, 32)

            return Number(((n >> 1n) as & 0xFFFFFFFFn) ^ (-((n & 0b1n) as & 0xFFFFFFFFn)))
        }
        function deserializeI64(de) {
            const n = try_take_varint(de, u64)

            return Number(((n >> 1n) & 0xFFFFFFFFFFFFFFFFn) ^ (-((n & 0b1n) & 0xFFFFFFFFFFFFFFFFn)))
        }
        function deserializeF32(de) {
            const bytes = de.try_take_n(4);

            const buf = new ArrayBuffer(4);
            const view = new DataView(buf);
        
            bytes.reverse().forEach((v, i) => view.setUint8(i, v));
        
            return view.getFloat32(0);
        }
        function deserializeF64(de) {
            const bytes = de.try_take_n(8);

            const buf = new ArrayBuffer(8);
            const view = new DataView(buf);
        
            bytes.reverse().forEach((v, i) => view.setUint8(i, v));
        
            return view.getFloat64(0);
        }
        function deserializeChar(de) {
            const sz = deserializeU64(de);
            if (sz > 4) {
                throw new Error("Deserialize bad char");
            }
            const bytes = de.try_take_n(Number(sz));
        
            const decoder = new TextDecoder('utf-8');
        
            return decoder.decode(bytes);
        }
        function deserializeString(de) {
            const sz = deserializeU64(de);
        
            let bytes = de.try_take_n(Number(sz));
        
            const decoder = new TextDecoder('utf-8');
        
            return decoder.decode(bytes);
        }
        function deserializeBytes(de) {
            const sz = deserializeU64(de);
        
            let bytes = de.try_take_n(Number(sz));
        
            const decoder = new TextDecoder('utf-8');
        
            return decoder.decode(bytes);
        }
        function deserializeList(de, inner) {
            const len = deserializeU64(de);
        
            let out: T[] = [];

            for (let i = 0; i < len; i++) {
                out.push(inner(de));   
            }
        
            return out;
        } 
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
                const disc = deserializeU32(de)

                switch (disc) {
                    case 0:
                return { tag: 0, value: null }
            case 1:
                return { tag: 1, value: deserializeU32(de) }
            case 2:
                return { tag: 2, value: deserializeString(de) }
            
                    default:
                        throw new Error("unknown variant case")
                }
        }function deserializeLoadStoreAllSizes(de) {
            return deserializeList(de, (de) => [deserializeString(de), deserializeU8(de), deserializeS8(de), deserializeU16(de), deserializeS16(de), deserializeU32(de), deserializeS32(de), deserializeU64(de), deserializeS64(de), deserializeF32(de), deserializeF64(de), deserializeChar(de)])
        }

            /**
* @param {Uint8Array[]} x 
*/
            export async function listU8Param (x) {
                return fetch('ipc://localhost/lists/list_u8_param', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @param {Uint16Array[]} x 
*/
            export async function listU16Param (x) {
                return fetch('ipc://localhost/lists/list_u16_param', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @param {Uint32Array[]} x 
*/
            export async function listU32Param (x) {
                return fetch('ipc://localhost/lists/list_u32_param', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @param {BigUint64Array[]} x 
*/
            export async function listU64Param (x) {
                return fetch('ipc://localhost/lists/list_u64_param', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @param {Int8Array[]} x 
*/
            export async function listS8Param (x) {
                return fetch('ipc://localhost/lists/list_s8_param', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @param {Int16Array[]} x 
*/
            export async function listS16Param (x) {
                return fetch('ipc://localhost/lists/list_s16_param', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @param {Int32Array[]} x 
*/
            export async function listS32Param (x) {
                return fetch('ipc://localhost/lists/list_s32_param', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @param {BigInt64Array[]} x 
*/
            export async function listS64Param (x) {
                return fetch('ipc://localhost/lists/list_s64_param', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @param {Float32Array[]} x 
*/
            export async function listFloat32Param (x) {
                return fetch('ipc://localhost/lists/list_float32_param', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @param {Float64Array[]} x 
*/
            export async function listFloat64Param (x) {
                return fetch('ipc://localhost/lists/list_float64_param', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @returns {Promise<Uint8Array[]>} 
*/
            export async function listU8Ret () {
                return fetch('ipc://localhost/lists/list_u8_ret', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeBytes(de)
                })
            }
        
            /**
* @returns {Promise<Uint16Array[]>} 
*/
            export async function listU16Ret () {
                return fetch('ipc://localhost/lists/list_u16_ret', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeList(de, (de) => deserializeU16(de))
                })
            }
        
            /**
* @returns {Promise<Uint32Array[]>} 
*/
            export async function listU32Ret () {
                return fetch('ipc://localhost/lists/list_u32_ret', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeList(de, (de) => deserializeU32(de))
                })
            }
        
            /**
* @returns {Promise<BigUint64Array[]>} 
*/
            export async function listU64Ret () {
                return fetch('ipc://localhost/lists/list_u64_ret', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeList(de, (de) => deserializeU64(de))
                })
            }
        
            /**
* @returns {Promise<Int8Array[]>} 
*/
            export async function listS8Ret () {
                return fetch('ipc://localhost/lists/list_s8_ret', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeList(de, (de) => deserializeS8(de))
                })
            }
        
            /**
* @returns {Promise<Int16Array[]>} 
*/
            export async function listS16Ret () {
                return fetch('ipc://localhost/lists/list_s16_ret', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeList(de, (de) => deserializeS16(de))
                })
            }
        
            /**
* @returns {Promise<Int32Array[]>} 
*/
            export async function listS32Ret () {
                return fetch('ipc://localhost/lists/list_s32_ret', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeList(de, (de) => deserializeS32(de))
                })
            }
        
            /**
* @returns {Promise<BigInt64Array[]>} 
*/
            export async function listS64Ret () {
                return fetch('ipc://localhost/lists/list_s64_ret', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeList(de, (de) => deserializeS64(de))
                })
            }
        
            /**
* @returns {Promise<Float32Array[]>} 
*/
            export async function listFloat32Ret () {
                return fetch('ipc://localhost/lists/list_float32_ret', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeList(de, (de) => deserializeF32(de))
                })
            }
        
            /**
* @returns {Promise<Float64Array[]>} 
*/
            export async function listFloat64Ret () {
                return fetch('ipc://localhost/lists/list_float64_ret', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeList(de, (de) => deserializeF64(de))
                })
            }
        
            /**
* @param {[number, number][]} x 
* @returns {Promise<[bigint, number][]>} 
*/
            export async function tupleList (x) {
                return fetch('ipc://localhost/lists/tuple_list', { method: "POST", body: JSON.stringify([x]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeList(de, (de) => [deserializeS64(de), deserializeU32(de)])
                })
            }
        
            /**
* @param {string[]} a 
*/
            export async function stringListArg (a) {
                return fetch('ipc://localhost/lists/string_list_arg', { method: "POST", body: JSON.stringify([a]) })
            }
        
            /**
* @returns {Promise<string[]>} 
*/
            export async function stringListRet () {
                return fetch('ipc://localhost/lists/string_list_ret', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeList(de, (de) => deserializeString(de))
                })
            }
        
            /**
* @param {[number, string][]} x 
* @returns {Promise<[string, number][]>} 
*/
            export async function tupleStringList (x) {
                return fetch('ipc://localhost/lists/tuple_string_list', { method: "POST", body: JSON.stringify([x]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeList(de, (de) => [deserializeString(de), deserializeU8(de)])
                })
            }
        
            /**
* @param {string[]} x 
* @returns {Promise<string[]>} 
*/
            export async function stringList (x) {
                return fetch('ipc://localhost/lists/string_list', { method: "POST", body: JSON.stringify([x]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeList(de, (de) => deserializeString(de))
                })
            }
        
            /**
* @param {SomeRecord[]} x 
* @returns {Promise<OtherRecord[]>} 
*/
            export async function recordList (x) {
                return fetch('ipc://localhost/lists/record_list', { method: "POST", body: JSON.stringify([x]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeList(de, (de) => deserializeOtherRecord(de))
                })
            }
        
            /**
* @param {OtherRecord[]} x 
* @returns {Promise<SomeRecord[]>} 
*/
            export async function recordListReverse (x) {
                return fetch('ipc://localhost/lists/record_list_reverse', { method: "POST", body: JSON.stringify([x]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeList(de, (de) => deserializeSomeRecord(de))
                })
            }
        
            /**
* @param {SomeVariant[]} x 
* @returns {Promise<OtherVariant[]>} 
*/
            export async function variantList (x) {
                return fetch('ipc://localhost/lists/variant_list', { method: "POST", body: JSON.stringify([x]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeList(de, (de) => deserializeOtherVariant(de))
                })
            }
        
            /**
* @param {LoadStoreAllSizes} a 
* @returns {Promise<LoadStoreAllSizes>} 
*/
            export async function loadStoreEverything (a) {
                return fetch('ipc://localhost/lists/load_store_everything', { method: "POST", body: JSON.stringify([a]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeLoadStoreAllSizes(de)
                })
            }
        
