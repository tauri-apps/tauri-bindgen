


            /**
 * A function that accepts a character 
*/
            export async function takeChar (x: string)  {
                return fetch('ipc://localhost/chars/take_char', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
 * A function that returns a character 
*/
            export async function returnChar () : Promise<string> {
                return fetch('ipc://localhost/chars/return_char', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        