


            
            export async function a (x: string)  {
                return fetch('ipc://localhost/strings/a', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function b () : Promise<string> {
                return fetch('ipc://localhost/strings/b', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function c (a: string, b: string) : Promise<string> {
                return fetch('ipc://localhost/strings/c', { method: "POST", body: JSON.stringify([a, b]) }).then(r => r.json())
            }
        