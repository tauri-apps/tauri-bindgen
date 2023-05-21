
            /**
* @param {Uint32Array[]} l 
*/
            export async function simpleList1 (l) {
                return fetch('ipc://localhost/simple_lists/simple_list1', { method: "POST", body: JSON.stringify([l]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<Uint32Array[]>} 
*/
            export async function simpleList2 () {
                return fetch('ipc://localhost/simple_lists/simple_list2', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @param {Uint32Array[]} a 
* @param {Uint32Array[]} b 
* @returns {Promise<[Uint32Array[], Uint32Array[]]>} 
*/
            export async function simpleList3 (a, b) {
                return fetch('ipc://localhost/simple_lists/simple_list3', { method: "POST", body: JSON.stringify([a, b]) }).then(r => r.json())
            }
        
            /**
* @param {Uint32Array[][]} l 
* @returns {Promise<Uint32Array[][]>} 
*/
            export async function simpleList4 (l) {
                return fetch('ipc://localhost/simple_lists/simple_list4', { method: "POST", body: JSON.stringify([l]) }).then(r => r.json())
            }
        
