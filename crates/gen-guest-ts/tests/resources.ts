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


class A {
                    #id: number;

                    
                        
                        async f1 ()  {
                        }
                    
                        
                        async f2 (a: number)  {
                        }
                    
                        
                        async f3 (a: number, b: number)  {
                        }
                    
                }
class B {
                    #id: number;

                    
                        
                        async f1 () Promise<A> {
                        }
                    
                        
                        async f2 (x: A) Promise<Result<number, null>> {
                        }
                    
                        
                        async f3 (x: A[] | null) Promise<Result<A, null>> {
                        }
                    
                }

            
            export async function constructorA () : Promise<A> {
                const out = []
                
                
                return fetch('ipc://localhost/resources/constructor_a', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return A.deserialize(de)
                }) as Promise<A>
            }
        
            
            export async function constructorB () : Promise<B> {
                const out = []
                
                
                return fetch('ipc://localhost/resources/constructor_b', { method: "POST", body: Uint8Array.from(out) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(new Uint8Array(bytes))

                    return B.deserialize(de)
                }) as Promise<B>
            }
        