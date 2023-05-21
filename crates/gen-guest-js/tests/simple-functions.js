
            /**
*/
            export async function f1 () {
                return fetch('ipc://localhost/simple_functions/f1', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @param {number} a 
*/
            export async function f2 (a) {
                return fetch('ipc://localhost/simple_functions/f2', { method: "POST", body: JSON.stringify([a]) }).then(r => r.json())
            }
        
            /**
* @param {number} a 
* @param {number} b 
*/
            export async function f3 (a, b) {
                return fetch('ipc://localhost/simple_functions/f3', { method: "POST", body: JSON.stringify([a, b]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<number>} 
*/
            export async function f4 () {
                return fetch('ipc://localhost/simple_functions/f4', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<[number, number]>} 
*/
            export async function f5 () {
                return fetch('ipc://localhost/simple_functions/f5', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @param {number} a 
* @param {number} b 
* @param {number} c 
* @returns {Promise<[number, number, number]>} 
*/
            export async function f6 (a, b, c) {
                return fetch('ipc://localhost/simple_functions/f6', { method: "POST", body: JSON.stringify([a, b, c]) }).then(r => r.json())
            }
        
