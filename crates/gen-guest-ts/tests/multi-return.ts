


            
            export async function mra ()  {
                return fetch('ipc://localhost/multi_return/mra', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function mrb ()  {
                return fetch('ipc://localhost/multi_return/mrb', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function mrc () : Promise<number> {
                return fetch('ipc://localhost/multi_return/mrc', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function mrd () : Promise<number> {
                return fetch('ipc://localhost/multi_return/mrd', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function mre () : Promise<[number, number]> {
                return fetch('ipc://localhost/multi_return/mre', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        