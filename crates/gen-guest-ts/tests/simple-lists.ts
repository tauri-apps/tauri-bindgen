


            
            export async function simpleList1 (l: Uint32Array[])  {
                return fetch('ipc://localhost/simple_lists/simple_list1', { method: "POST", body: JSON.stringify([l]) }).then(r => r.json())
            }
        
            
            export async function simpleList2 () : Promise<Uint32Array[]> {
                return fetch('ipc://localhost/simple_lists/simple_list2', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function simpleList3 (a: Uint32Array[], b: Uint32Array[]) : Promise<[Uint32Array[], Uint32Array[]]> {
                return fetch('ipc://localhost/simple_lists/simple_list3', { method: "POST", body: JSON.stringify([a, b]) }).then(r => r.json())
            }
        
            
            export async function simpleList4 (l: Uint32Array[][]) : Promise<Uint32Array[][]> {
                return fetch('ipc://localhost/simple_lists/simple_list4', { method: "POST", body: JSON.stringify([l]) }).then(r => r.json())
            }
        