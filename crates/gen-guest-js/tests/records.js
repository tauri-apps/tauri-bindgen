
            /**
* @param {[string, number]} x 
*/
            export async function tupleArg (x) {
                return fetch('ipc://localhost/records/tuple_arg', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<[string, number]>} 
*/
            export async function tupleResult () {
                return fetch('ipc://localhost/records/tuple_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @param {Empty} x 
*/
            export async function emptyArg (x) {
                return fetch('ipc://localhost/records/empty_arg', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<Empty>} 
*/
            export async function emptyResult () {
                return fetch('ipc://localhost/records/empty_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @param {Scalars} x 
*/
            export async function scalarArg (x) {
                return fetch('ipc://localhost/records/scalar_arg', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<Scalars>} 
*/
            export async function scalarResult () {
                return fetch('ipc://localhost/records/scalar_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @param {ReallyFlags} x 
*/
            export async function flagsArg (x) {
                return fetch('ipc://localhost/records/flags_arg', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<ReallyFlags>} 
*/
            export async function flagsResult () {
                return fetch('ipc://localhost/records/flags_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @param {Aggregates} x 
*/
            export async function aggregateArg (x) {
                return fetch('ipc://localhost/records/aggregate_arg', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<Aggregates>} 
*/
            export async function aggregateResult () {
                return fetch('ipc://localhost/records/aggregate_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @param {TupleTypedef2} e 
* @returns {Promise<number>} 
*/
            export async function typedefInout (e) {
                return fetch('ipc://localhost/records/typedef_inout', { method: "POST", body: JSON.stringify([e]) }).then(r => r.json())
            }
        
