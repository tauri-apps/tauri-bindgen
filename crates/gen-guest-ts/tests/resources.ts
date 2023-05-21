

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

                    
                        
                        async f1 () : Promise<A> {
                        }
                    
                        
                        async f2 (x: A) : Promise<Result<number, _>> {
                        }
                    
                        
                        async f3 (x: A[] | null) : Promise<Result<A, _>> {
                        }
                    
                }

            
            export async function constructorA () : Promise<A> {
                return fetch('ipc://localhost/resources/constructor_a', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function constructorB () : Promise<B> {
                return fetch('ipc://localhost/resources/constructor_b', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        