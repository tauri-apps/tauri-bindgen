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
        

            /**
* @param {number} x 
*/
            export async function float32Param (x) {
                return fetch('ipc://localhost/floats/float32_param', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @param {number} x 
*/
            export async function float64Param (x) {
                return fetch('ipc://localhost/floats/float64_param', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
* @returns {Promise<number>} 
*/
            export async function float32Result () {
                return fetch('ipc://localhost/floats/float32_result', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeF32(de)
                })
            }
        
            /**
* @returns {Promise<number>} 
*/
            export async function float64Result () {
                return fetch('ipc://localhost/floats/float64_result', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return deserializeF64(de)
                })
            }
        
