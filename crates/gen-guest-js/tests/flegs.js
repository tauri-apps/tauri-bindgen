
            /**
* @param {Flag1} x 
* @returns {Promise<Flag1>} 
*/
            export async function roundtripFlag1 (x) {
                return fetch('ipc://localhost/flegs/roundtrip_flag1', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {Flag2} x 
* @returns {Promise<Flag2>} 
*/
            export async function roundtripFlag2 (x) {
                return fetch('ipc://localhost/flegs/roundtrip_flag2', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {Flag4} x 
* @returns {Promise<Flag4>} 
*/
            export async function roundtripFlag4 (x) {
                return fetch('ipc://localhost/flegs/roundtrip_flag4', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {Flag8} x 
* @returns {Promise<Flag8>} 
*/
            export async function roundtripFlag8 (x) {
                return fetch('ipc://localhost/flegs/roundtrip_flag8', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {Flag16} x 
* @returns {Promise<Flag16>} 
*/
            export async function roundtripFlag16 (x) {
                return fetch('ipc://localhost/flegs/roundtrip_flag16', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {Flag32} x 
* @returns {Promise<Flag32>} 
*/
            export async function roundtripFlag32 (x) {
                return fetch('ipc://localhost/flegs/roundtrip_flag32', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {Flag64} x 
* @returns {Promise<Flag64>} 
*/
            export async function roundtripFlag64 (x) {
                return fetch('ipc://localhost/flegs/roundtrip_flag64', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
