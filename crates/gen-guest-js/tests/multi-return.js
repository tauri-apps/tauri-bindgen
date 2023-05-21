
            /**
*/
            export async function mra () {
                return fetch('ipc://localhost/multi_return/mra', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<[]>} 
*/
            export async function mrb () {
                return fetch('ipc://localhost/multi_return/mrb', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<number>} 
*/
            export async function mrc () {
                return fetch('ipc://localhost/multi_return/mrc', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<[number]>} 
*/
            export async function mrd () {
                return fetch('ipc://localhost/multi_return/mrd', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<[number, number]>} 
*/
            export async function mre () {
                return fetch('ipc://localhost/multi_return/mre', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
