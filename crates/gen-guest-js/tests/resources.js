
            /**
* @returns {Promise<A>} 
*/
            export async function constructorA () {
            }
        
            /**
* @returns {Promise<B>} 
*/
            export async function constructorB () {
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