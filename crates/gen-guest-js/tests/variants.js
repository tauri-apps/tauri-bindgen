
            /**
* @param {E1} x 
*/
            export async function e1Arg (x) {
                return fetch('ipc://localhost/variants/e1_arg', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<E1>} 
*/
            export async function e1Result () {
                return fetch('ipc://localhost/variants/e1_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @param {U1} x 
*/
            export async function u1Arg (x) {
                return fetch('ipc://localhost/variants/u1_arg', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<U1>} 
*/
            export async function u1Result () {
                return fetch('ipc://localhost/variants/u1_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @param {V1} x 
*/
            export async function v1Arg (x) {
                return fetch('ipc://localhost/variants/v1_arg', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<V1>} 
*/
            export async function v1Result () {
                return fetch('ipc://localhost/variants/v1_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @param {boolean} x 
*/
            export async function boolArg (x) {
                return fetch('ipc://localhost/variants/bool_arg', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<boolean>} 
*/
            export async function boolResult () {
                return fetch('ipc://localhost/variants/bool_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @param {boolean | null} a 
* @param {[] | null} b 
* @param {number | null} c 
* @param {E1 | null} d 
* @param {number | null} e 
* @param {U1 | null} f 
* @param {boolean | null | null} g 
*/
            export async function optionArg (a, b, c, d, e, f, g) {
                return fetch('ipc://localhost/variants/option_arg', { method: "POST", body: JSON.stringify([a, b, c, d, e, f, g]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<[boolean | null, [] | null, number | null, E1 | null, number | null, U1 | null, boolean | null | null]>} 
*/
            export async function optionResult () {
                return fetch('ipc://localhost/variants/option_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @param {Casts1} a 
* @param {Casts2} b 
* @param {Casts3} c 
* @param {Casts4} d 
* @param {Casts5} e 
* @param {Casts6} f 
* @returns {Promise<[Casts1, Casts2, Casts3, Casts4, Casts5, Casts6]>} 
*/
            export async function casts (a, b, c, d, e, f) {
                return fetch('ipc://localhost/variants/casts', { method: "POST", body: JSON.stringify([a, b, c, d, e, f]) }).then(r => r.json())
            }
        
            /**
* @param {Result<_, _>} a 
* @param {Result<_, E1>} b 
* @param {Result<E1, _>} c 
* @param {Result<[], []>} d 
* @param {Result<number, V1>} e 
* @param {Result<string, Uint8Array[]>} f 
*/
            export async function resultArg (a, b, c, d, e, f) {
                return fetch('ipc://localhost/variants/result_arg', { method: "POST", body: JSON.stringify([a, b, c, d, e, f]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<[Result<_, _>, Result<_, E1>, Result<E1, _>, Result<[], []>, Result<number, V1>, Result<string, Uint8Array[]>]>} 
*/
            export async function resultResult () {
                return fetch('ipc://localhost/variants/result_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<Result<number, MyErrno>>} 
*/
            export async function returnResultSugar () {
                return fetch('ipc://localhost/variants/return_result_sugar', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<Result<_, MyErrno>>} 
*/
            export async function returnResultSugar2 () {
                return fetch('ipc://localhost/variants/return_result_sugar2', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<Result<MyErrno, MyErrno>>} 
*/
            export async function returnResultSugar3 () {
                return fetch('ipc://localhost/variants/return_result_sugar3', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<Result<[number, number], MyErrno>>} 
*/
            export async function returnResultSugar4 () {
                return fetch('ipc://localhost/variants/return_result_sugar4', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<number | null>} 
*/
            export async function returnOptionSugar () {
                return fetch('ipc://localhost/variants/return_option_sugar', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<MyErrno | null>} 
*/
            export async function returnOptionSugar2 () {
                return fetch('ipc://localhost/variants/return_option_sugar2', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<Result<number, number>>} 
*/
            export async function resultSimple () {
                return fetch('ipc://localhost/variants/result_simple', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @param {IsClone} a 
*/
            export async function isCloneArg (a) {
                return fetch('ipc://localhost/variants/is_clone_arg', { method: "POST", body: JSON.stringify([a]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<IsClone>} 
*/
            export async function isCloneReturn () {
                return fetch('ipc://localhost/variants/is_clone_return', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<[number | null]>} 
*/
            export async function returnNamedOption () {
                return fetch('ipc://localhost/variants/return_named_option', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<[Result<number, MyErrno>]>} 
*/
            export async function returnNamedResult () {
                return fetch('ipc://localhost/variants/return_named_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
