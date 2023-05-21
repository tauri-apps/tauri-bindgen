
            /**
* @param {AllIntegers} num 
* @returns {Promise<AllIntegers>} 
*/
            export async function addOneInteger (num) {
                return fetch('ipc://localhost/unions/add_one_integer', { method: "POST", body: JSON.stringify([num]) }).then(r => r.json())
            }
        
            /**
* @param {AllFloats} num 
* @returns {Promise<AllFloats>} 
*/
            export async function addOneFloat (num) {
                return fetch('ipc://localhost/unions/add_one_float', { method: "POST", body: JSON.stringify([num]) }).then(r => r.json())
            }
        
            /**
* @param {AllText} text 
* @param {string} letter 
* @returns {Promise<AllText>} 
*/
            export async function replaceFirstChar (text, letter) {
                return fetch('ipc://localhost/unions/replace_first_char', { method: "POST", body: JSON.stringify([text, letter]) }).then(r => r.json())
            }
        
            /**
* @param {AllIntegers} num 
* @returns {Promise<number>} 
*/
            export async function identifyInteger (num) {
                return fetch('ipc://localhost/unions/identify_integer', { method: "POST", body: JSON.stringify([num]) }).then(r => r.json())
            }
        
            /**
* @param {AllFloats} num 
* @returns {Promise<number>} 
*/
            export async function identifyFloat (num) {
                return fetch('ipc://localhost/unions/identify_float', { method: "POST", body: JSON.stringify([num]) }).then(r => r.json())
            }
        
            /**
* @param {AllText} text 
* @returns {Promise<number>} 
*/
            export async function identifyText (text) {
                return fetch('ipc://localhost/unions/identify_text', { method: "POST", body: JSON.stringify([text]) }).then(r => r.json())
            }
        
            /**
* @param {DuplicatedS32} num 
* @returns {Promise<DuplicatedS32>} 
*/
            export async function addOneDuplicated (num) {
                return fetch('ipc://localhost/unions/add_one_duplicated', { method: "POST", body: JSON.stringify([num]) }).then(r => r.json())
            }
        
            /**
* @param {DuplicatedS32} num 
* @returns {Promise<number>} 
*/
            export async function identifyDuplicated (num) {
                return fetch('ipc://localhost/unions/identify_duplicated', { method: "POST", body: JSON.stringify([num]) }).then(r => r.json())
            }
        
            /**
* @param {DistinguishableNum} num 
* @returns {Promise<DistinguishableNum>} 
*/
            export async function addOneDistinguishableNum (num) {
                return fetch('ipc://localhost/unions/add_one_distinguishable_num', { method: "POST", body: JSON.stringify([num]) }).then(r => r.json())
            }
        
            /**
* @param {DistinguishableNum} num 
* @returns {Promise<number>} 
*/
            export async function identifyDistinguishableNum (num) {
                return fetch('ipc://localhost/unions/identify_distinguishable_num', { method: "POST", body: JSON.stringify([num]) }).then(r => r.json())
            }
        
