


            
            export async function f1 ()  {
                return fetch('ipc://localhost/simple_functions/f1', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function f2 (a: number)  {
                return fetch('ipc://localhost/simple_functions/f2', { method: "POST", body: JSON.stringify([a]) }).then(r => r.json())
            }
        
            
            export async function f3 (a: number, b: number)  {
                return fetch('ipc://localhost/simple_functions/f3', { method: "POST", body: JSON.stringify([a, b]) }).then(r => r.json())
            }
        
            
            export async function f4 () : Promise<number> {
                return fetch('ipc://localhost/simple_functions/f4', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function f5 () : Promise<[number, number]> {
                return fetch('ipc://localhost/simple_functions/f5', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function f6 (a: number, b: number, c: number) : Promise<[number, number, number]> {
                return fetch('ipc://localhost/simple_functions/f6', { method: "POST", body: JSON.stringify([a, b, c]) }).then(r => r.json())
            }
        