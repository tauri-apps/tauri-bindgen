
            /**
* @param {string} x 
*/
            export async function a (x) {
                return fetch('ipc://localhost/strings/a', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<string>} 
*/
            export async function b () {
                return fetch('ipc://localhost/strings/b', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @param {string} a 
* @param {string} b 
* @returns {Promise<string>} 
*/
            export async function c (a, b) {
                return fetch('ipc://localhost/strings/c', { method: "POST", body: JSON.stringify([a, b]) }).then(r => r.json())
            }
        
