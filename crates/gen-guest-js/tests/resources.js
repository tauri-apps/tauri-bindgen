
            /**
* @returns {Promise<A>} 
*/
            export async function constructorA () {
                return fetch('ipc://localhost/resources/constructor_a', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<B>} 
*/
            export async function constructorB () {
                return fetch('ipc://localhost/resources/constructor_b', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        

class a {
            #id: number;
    
            
                /**
*/
                async f1 () {
                }
            
                /**
* @param {number} a 
*/
                async f2 (a) {
                }
            
                /**
* @param {number} a 
* @param {number} b 
*/
                async f3 (a, b) {
                }
            
        }
class b {
            #id: number;
    
            
                /**
* @returns {Promise<A>} 
*/
                async f1 () {
                }
            
                /**
* @param {A} x 
* @returns {Promise<Result<number, _>>} 
*/
                async f2 (x) {
                }
            
                /**
* @param {A[] | null} x 
* @returns {Promise<Result<A, _>>} 
*/
                async f3 (x) {
                }
            
        }