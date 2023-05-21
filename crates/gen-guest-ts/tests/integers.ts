


            
            export async function a1 (x: number)  {
                return fetch('ipc://localhost/integers/a1', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function a2 (x: number)  {
                return fetch('ipc://localhost/integers/a2', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function a3 (x: number)  {
                return fetch('ipc://localhost/integers/a3', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function a4 (x: number)  {
                return fetch('ipc://localhost/integers/a4', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function a5 (x: number)  {
                return fetch('ipc://localhost/integers/a5', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function a6 (x: number)  {
                return fetch('ipc://localhost/integers/a6', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function a7 (x: bigint)  {
                return fetch('ipc://localhost/integers/a7', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function a8 (x: bigint)  {
                return fetch('ipc://localhost/integers/a8', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function a9 (p1: number, p2: number, p3: number, p4: number, p5: number, p6: number, p7: bigint, p8: bigint)  {
                return fetch('ipc://localhost/integers/a9', { method: "POST", body: JSON.stringify([p1, p2, p3, p4, p5, p6, p7, p8]) }).then(r => r.json())
            }
        
            
            export async function r1 () : Promise<number> {
                return fetch('ipc://localhost/integers/r1', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function r2 () : Promise<number> {
                return fetch('ipc://localhost/integers/r2', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function r3 () : Promise<number> {
                return fetch('ipc://localhost/integers/r3', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function r4 () : Promise<number> {
                return fetch('ipc://localhost/integers/r4', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function r5 () : Promise<number> {
                return fetch('ipc://localhost/integers/r5', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function r6 () : Promise<number> {
                return fetch('ipc://localhost/integers/r6', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function r7 () : Promise<bigint> {
                return fetch('ipc://localhost/integers/r7', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function r8 () : Promise<bigint> {
                return fetch('ipc://localhost/integers/r8', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function pairRet () : Promise<[bigint, number]> {
                return fetch('ipc://localhost/integers/pair_ret', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        