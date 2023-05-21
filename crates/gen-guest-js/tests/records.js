export class Deserializer {
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
        function deserializeBool(de) {
            const val = de.pop();
        
            return val != 0
        }
        function deserializeU32(de) {
            return try_take_varint(de, 32)
        }
        function deserializeU64(de) {
            return try_take_varint(de, 64)
        }
        function deserializeI32(de) {
            const n = try_take_varint(de, 32)

            return Number(((n >> 1n) as & 0xFFFFFFFFn) ^ (-((n & 0b1n) as & 0xFFFFFFFFn)))
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
                a: deserializeBoolean(de),
b: deserializeBoolean(de),
c: deserializeBoolean(de),
d: deserializeBoolean(de),
e: deserializeBoolean(de),
f: deserializeBoolean(de),
g: deserializeBoolean(de),
h: deserializeBoolean(de),
i: deserializeBoolean(de)
            }
        }function deserializeAggregates(de) {
            return {
                a: deserializeScalars(de),
b: deserializeU32(de),
c: deserializeEmpty(de),
d: deserializeString(de),
e: deserializeReallyFlags(de)
            }
        }

            /**
* @param {[string, number]} x 
*/
            export async function tupleArg (x) {
                return fetch('ipc://localhost/records/tuple_arg', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @returns {Promise<[string, number]>} 
*/
            export async function tupleResult () {
                return fetch('ipc://localhost/records/tuple_result', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return [deserializeChar(de), deserializeU32(de)]
                })
            }
        
            /**
* @param {Empty} x 
*/
            export async function emptyArg (x) {
                return fetch('ipc://localhost/records/empty_arg', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @returns {Promise<Empty>} 
*/
            export async function emptyResult () {
                return fetch('ipc://localhost/records/empty_result', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeEmpty(de)
                })
            }
        
            /**
* @param {Scalars} x 
*/
            export async function scalarArg (x) {
                return fetch('ipc://localhost/records/scalar_arg', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @returns {Promise<Scalars>} 
*/
            export async function scalarResult () {
                return fetch('ipc://localhost/records/scalar_result', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeScalars(de)
                })
            }
        
            /**
* @param {ReallyFlags} x 
*/
            export async function flagsArg (x) {
                return fetch('ipc://localhost/records/flags_arg', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @returns {Promise<ReallyFlags>} 
*/
            export async function flagsResult () {
                return fetch('ipc://localhost/records/flags_result', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeReallyFlags(de)
                })
            }
        
            /**
* @param {Aggregates} x 
*/
            export async function aggregateArg (x) {
                return fetch('ipc://localhost/records/aggregate_arg', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @returns {Promise<Aggregates>} 
*/
            export async function aggregateResult () {
                return fetch('ipc://localhost/records/aggregate_result', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeAggregates(de)
                })
            }
        
            /**
* @param {TupleTypedef2} e 
* @returns {Promise<number>} 
*/
            export async function typedefInout (e) {
                return fetch('ipc://localhost/records/typedef_inout', { method: "POST", body: JSON.stringify([e]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeS32(de)
                })
            }
        
