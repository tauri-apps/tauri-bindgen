


            
            export async function float32Param (x: number)  {
                return fetch('ipc://localhost/floats/float32_param', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function float64Param (x: number)  {
                return fetch('ipc://localhost/floats/float64_param', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function float32Result () : Promise<number> {
                return fetch('ipc://localhost/floats/float32_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function float64Result () : Promise<number> {
                return fetch('ipc://localhost/floats/float64_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        