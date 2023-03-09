
            /**
* @param {E1} x 
* @returns {Promise<[]>} 
*/
            export async function e1Arg (x) {
            }
        

            /**
* @returns {Promise<E1>} 
*/
            export async function e1Result () {
            }
        

            /**
* @param {U1} x 
* @returns {Promise<[]>} 
*/
            export async function u1Arg (x) {
            }
        

            /**
* @returns {Promise<U1>} 
*/
            export async function u1Result () {
            }
        

            /**
* @param {V1} x 
* @returns {Promise<[]>} 
*/
            export async function v1Arg (x) {
            }
        

            /**
* @returns {Promise<V1>} 
*/
            export async function v1Result () {
            }
        

            /**
* @param {boolean} x 
* @returns {Promise<[]>} 
*/
            export async function boolArg (x) {
            }
        

            /**
* @returns {Promise<boolean>} 
*/
            export async function boolResult () {
            }
        

            /**
* @param {boolean | null} a 
* @param {[] | null} b 
* @param {number | null} c 
* @param {E1 | null} d 
* @param {number | null} e 
* @param {U1 | null} f 
* @param {boolean | null | null} g 
* @returns {Promise<[]>} 
*/
            export async function optionArg (a, b, c, d, e, f, g) {
            }
        

            /**
* @returns {Promise<[boolean | null, [] | null, number | null, E1 | null, number | null, U1 | null, boolean | null | null]>} 
*/
            export async function optionResult () {
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
            }
        

            /**
* @param {Result<_, _>} a 
* @param {Result<_, E1>} b 
* @param {Result<E1, _>} c 
* @param {Result<[], []>} d 
* @param {Result<number, V1>} e 
* @param {Result<string, Uint8Array[]>} f 
* @returns {Promise<[]>} 
*/
            export async function resultArg (a, b, c, d, e, f) {
            }
        

            /**
* @returns {Promise<[Result<_, _>, Result<_, E1>, Result<E1, _>, Result<[], []>, Result<number, V1>, Result<string, Uint8Array[]>]>} 
*/
            export async function resultResult () {
            }
        

            /**
* @returns {Promise<Result<number, MyErrno>>} 
*/
            export async function returnResultSugar () {
            }
        

            /**
* @returns {Promise<Result<_, MyErrno>>} 
*/
            export async function returnResultSugar2 () {
            }
        

            /**
* @returns {Promise<Result<MyErrno, MyErrno>>} 
*/
            export async function returnResultSugar3 () {
            }
        

            /**
* @returns {Promise<Result<[number, number], MyErrno>>} 
*/
            export async function returnResultSugar4 () {
            }
        

            /**
* @returns {Promise<number | null>} 
*/
            export async function returnOptionSugar () {
            }
        

            /**
* @returns {Promise<MyErrno | null>} 
*/
            export async function returnOptionSugar2 () {
            }
        

            /**
* @returns {Promise<Result<number, number>>} 
*/
            export async function resultSimple () {
            }
        

            /**
* @param {IsClone} a 
* @returns {Promise<[]>} 
*/
            export async function isCloneArg (a) {
            }
        

            /**
* @returns {Promise<IsClone>} 
*/
            export async function isCloneReturn () {
            }
        

            /**
* @returns {Promise<number | null>} 
*/
            export async function returnNamedOption () {
            }
        

            /**
* @returns {Promise<Result<number, MyErrno>>} 
*/
            export async function returnNamedResult () {
            }
        