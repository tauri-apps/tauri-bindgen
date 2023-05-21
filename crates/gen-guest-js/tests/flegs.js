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
        }

            /**
* @param {Flag1} x 
* @returns {Promise<Flag1>} 
*/
            export async function roundtripFlag1 (x) {
                return fetch('ipc://localhost/flegs/roundtrip_flag1', { method: "POST", body: JSON.stringify([x]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeFlag1(de)
                })
            }
        
            /**
* @param {Flag2} x 
* @returns {Promise<Flag2>} 
*/
            export async function roundtripFlag2 (x) {
                return fetch('ipc://localhost/flegs/roundtrip_flag2', { method: "POST", body: JSON.stringify([x]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeFlag2(de)
                })
            }
        
            /**
* @param {Flag4} x 
* @returns {Promise<Flag4>} 
*/
            export async function roundtripFlag4 (x) {
                return fetch('ipc://localhost/flegs/roundtrip_flag4', { method: "POST", body: JSON.stringify([x]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeFlag4(de)
                })
            }
        
            /**
* @param {Flag8} x 
* @returns {Promise<Flag8>} 
*/
            export async function roundtripFlag8 (x) {
                return fetch('ipc://localhost/flegs/roundtrip_flag8', { method: "POST", body: JSON.stringify([x]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeFlag8(de)
                })
            }
        
            /**
* @param {Flag16} x 
* @returns {Promise<Flag16>} 
*/
            export async function roundtripFlag16 (x) {
                return fetch('ipc://localhost/flegs/roundtrip_flag16', { method: "POST", body: JSON.stringify([x]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeFlag16(de)
                })
            }
        
            /**
* @param {Flag32} x 
* @returns {Promise<Flag32>} 
*/
            export async function roundtripFlag32 (x) {
                return fetch('ipc://localhost/flegs/roundtrip_flag32', { method: "POST", body: JSON.stringify([x]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeFlag32(de)
                })
            }
        
            /**
* @param {Flag64} x 
* @returns {Promise<Flag64>} 
*/
            export async function roundtripFlag64 (x) {
                return fetch('ipc://localhost/flegs/roundtrip_flag64', { method: "POST", body: JSON.stringify([x]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeFlag64(de)
                })
            }
        
