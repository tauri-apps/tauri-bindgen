
            /**
* @param {number} x 
*/
            export async function float32Param (x) {
                return fetch('ipc://localhost/floats/float32_param', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {number} x 
*/
            export async function float64Param (x) {
                return fetch('ipc://localhost/floats/float64_param', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<number>} 
*/
            export async function float32Result () {
                return fetch('ipc://localhost/floats/float32_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<number>} 
*/
            export async function float64Result () {
                return fetch('ipc://localhost/floats/float64_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
