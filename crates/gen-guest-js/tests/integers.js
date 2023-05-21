
            /**
* @param {number} x 
*/
            export async function a1 (x) {
                return fetch('ipc://localhost/integers/a1', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {number} x 
*/
            export async function a2 (x) {
                return fetch('ipc://localhost/integers/a2', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {number} x 
*/
            export async function a3 (x) {
                return fetch('ipc://localhost/integers/a3', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {number} x 
*/
            export async function a4 (x) {
                return fetch('ipc://localhost/integers/a4', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {number} x 
*/
            export async function a5 (x) {
                return fetch('ipc://localhost/integers/a5', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {number} x 
*/
            export async function a6 (x) {
                return fetch('ipc://localhost/integers/a6', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {bigint} x 
*/
            export async function a7 (x) {
                return fetch('ipc://localhost/integers/a7', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {bigint} x 
*/
            export async function a8 (x) {
                return fetch('ipc://localhost/integers/a8', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {number} p1 
* @param {number} p2 
* @param {number} p3 
* @param {number} p4 
* @param {number} p5 
* @param {number} p6 
* @param {bigint} p7 
* @param {bigint} p8 
*/
            export async function a9 (p1, p2, p3, p4, p5, p6, p7, p8) {
                return fetch('ipc://localhost/integers/a9', { method: "POST", body: JSON.stringify([p1, p2, p3, p4, p5, p6, p7, p8]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<number>} 
*/
            export async function r1 () {
                return fetch('ipc://localhost/integers/r1', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<number>} 
*/
            export async function r2 () {
                return fetch('ipc://localhost/integers/r2', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<number>} 
*/
            export async function r3 () {
                return fetch('ipc://localhost/integers/r3', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<number>} 
*/
            export async function r4 () {
                return fetch('ipc://localhost/integers/r4', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<number>} 
*/
            export async function r5 () {
                return fetch('ipc://localhost/integers/r5', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<number>} 
*/
            export async function r6 () {
                return fetch('ipc://localhost/integers/r6', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<bigint>} 
*/
            export async function r7 () {
                return fetch('ipc://localhost/integers/r7', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<bigint>} 
*/
            export async function r8 () {
                return fetch('ipc://localhost/integers/r8', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<[bigint, number]>} 
*/
            export async function pairRet () {
                return fetch('ipc://localhost/integers/pair_ret', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
