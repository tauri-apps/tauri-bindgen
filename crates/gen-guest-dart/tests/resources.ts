

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
            }
        
            
            export async function constructorB () : Promise<B> {
            }
        