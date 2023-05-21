
            /**
 * A function that accepts a character 
* @param {string} x 
*/
            export async function takeChar (x) {
                return fetch('ipc://localhost/chars/take_char', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
 * A function that returns a character 
* @returns {Promise<string>} 
*/
            export async function returnChar () {
                return fetch('ipc://localhost/chars/return_char', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
