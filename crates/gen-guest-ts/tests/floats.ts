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
function deserializeF32(de) {
    const bytes = de.try_take_n(4);

    const buf = new ArrayBuffer(4);
    const view = new DataView(buf);

    bytes.forEach((v, i) => view.setUint8(i, v));

    return view.getFloat32(0, true);
}function deserializeF64(de) {
    const bytes = de.try_take_n(8);

    const buf = new ArrayBuffer(8);
    const view = new DataView(buf);

    bytes.forEach((v, i) => view.setUint8(i, v));

    return view.getFloat64(0, true);
}function serializeF32(out, val) {
    const buf = new ArrayBuffer(4);
    const view = new DataView(buf);

    view.setFloat32(0, val, true);

    out.push(...new Uint8Array(buf))
}function serializeF64(out, val) {
    const buf = new ArrayBuffer(8);
    const view = new DataView(buf);

    view.setFloat64(0, val, true);

    out.push(...new Uint8Array(buf))
}


            
            export async function float32Param (x: number) : Promise<void> {
                const out = []
                serializeF32(out, x)
                
                 fetch('ipc://localhost/floats/float32_param', { method: "POST", body: Uint8Array.from(out) }) 
            }
        
            
            export async function float64Param (x: number) : Promise<void> {
                const out = []
                serializeF64(out, x)
                
                 fetch('ipc://localhost/floats/float64_param', { method: "POST", body: Uint8Array.from(out) }) 
            }
        
            
            export async function float32Result () : Promise<number> {
                const out = []
                
                
                return fetch('ipc://localhost/floats/float32_result', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeF32(de)
                }) as Promise<number>
            }
        
            
            export async function float64Result () : Promise<number> {
                const out = []
                
                
                return fetch('ipc://localhost/floats/float64_result', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return deserializeF64(de)
                }) as Promise<number>
            }
        