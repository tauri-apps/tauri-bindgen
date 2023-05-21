
            /**
* @returns {Promise<Result<string | null, Error>>} 
*/
            export async function optionTest () {
                return fetch('ipc://localhost/small_anonymous/option_test', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
