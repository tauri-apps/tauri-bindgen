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
    

            /**
* @returns {Promise<A>} 
*/
            export async function constructorA () {
                return fetch('ipc://localhost/resources/constructor_a', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return A.deserialize(de)
                })
            }
        
            /**
* @returns {Promise<B>} 
*/
            export async function constructorB () {
                return fetch('ipc://localhost/resources/constructor_b', { method: "POST", body: JSON.stringify([]) })
                .then(r => r.arrayBuffer())
                .then(bytes => {
                    const de = new Deserializer(Uint8Array.from(bytes))

                    return B.deserialize(de)
                })
            }
        

class A {
            #id;
            
                /**
*/
                async f1 () {
                }
            
                /**
* @param {number} a 
*/
                async f2 (a) {
                }
            
                /**
* @param {number} a 
* @param {number} b 
*/
                async f3 (a, b) {
                }
            
            deserialize(de) {
                            const self = new A();
                            self.#id = deserializeU64(de);
                            return self
                        }
        }
class B {
            #id;
            
                /**
* @returns {Promise<A>} 
*/
                async f1 () {
                }
            
                /**
* @param {A} x 
* @returns {Promise<Result<number, _>>} 
*/
                async f2 (x) {
                }
            
                /**
* @param {A[] | null} x 
* @returns {Promise<Result<A, _>>} 
*/
                async f3 (x) {
                }
            
            deserialize(de) {
                            const self = new B();
                            self.#id = deserializeU64(de);
                            return self
                        }
        }