
            /**
* @param {Uint8Array[]} x 
*/
            export async function listU8Param (x) {
                return fetch('ipc://localhost/lists/list_u8_param', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {Uint16Array[]} x 
*/
            export async function listU16Param (x) {
                return fetch('ipc://localhost/lists/list_u16_param', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {Uint32Array[]} x 
*/
            export async function listU32Param (x) {
                return fetch('ipc://localhost/lists/list_u32_param', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {BigUint64Array[]} x 
*/
            export async function listU64Param (x) {
                return fetch('ipc://localhost/lists/list_u64_param', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {Int8Array[]} x 
*/
            export async function listS8Param (x) {
                return fetch('ipc://localhost/lists/list_s8_param', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {Int16Array[]} x 
*/
            export async function listS16Param (x) {
                return fetch('ipc://localhost/lists/list_s16_param', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {Int32Array[]} x 
*/
            export async function listS32Param (x) {
                return fetch('ipc://localhost/lists/list_s32_param', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {BigInt64Array[]} x 
*/
            export async function listS64Param (x) {
                return fetch('ipc://localhost/lists/list_s64_param', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {Float32Array[]} x 
*/
            export async function listFloat32Param (x) {
                return fetch('ipc://localhost/lists/list_float32_param', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {Float64Array[]} x 
*/
            export async function listFloat64Param (x) {
                return fetch('ipc://localhost/lists/list_float64_param', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<Uint8Array[]>} 
*/
            export async function listU8Ret () {
                return fetch('ipc://localhost/lists/list_u8_ret', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<Uint16Array[]>} 
*/
            export async function listU16Ret () {
                return fetch('ipc://localhost/lists/list_u16_ret', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<Uint32Array[]>} 
*/
            export async function listU32Ret () {
                return fetch('ipc://localhost/lists/list_u32_ret', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<BigUint64Array[]>} 
*/
            export async function listU64Ret () {
                return fetch('ipc://localhost/lists/list_u64_ret', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<Int8Array[]>} 
*/
            export async function listS8Ret () {
                return fetch('ipc://localhost/lists/list_s8_ret', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<Int16Array[]>} 
*/
            export async function listS16Ret () {
                return fetch('ipc://localhost/lists/list_s16_ret', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<Int32Array[]>} 
*/
            export async function listS32Ret () {
                return fetch('ipc://localhost/lists/list_s32_ret', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<BigInt64Array[]>} 
*/
            export async function listS64Ret () {
                return fetch('ipc://localhost/lists/list_s64_ret', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<Float32Array[]>} 
*/
            export async function listFloat32Ret () {
                return fetch('ipc://localhost/lists/list_float32_ret', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<Float64Array[]>} 
*/
            export async function listFloat64Ret () {
                return fetch('ipc://localhost/lists/list_float64_ret', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @param {[number, number][]} x 
* @returns {Promise<[bigint, number][]>} 
*/
            export async function tupleList (x) {
                return fetch('ipc://localhost/lists/tuple_list', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {string[]} a 
*/
            export async function stringListArg (a) {
                return fetch('ipc://localhost/lists/string_list_arg', { method: "POST", body: JSON.stringify([a]) }).then(r => r.json())
            }
        
            /**
* @returns {Promise<string[]>} 
*/
            export async function stringListRet () {
                return fetch('ipc://localhost/lists/string_list_ret', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            /**
* @param {[number, string][]} x 
* @returns {Promise<[string, number][]>} 
*/
            export async function tupleStringList (x) {
                return fetch('ipc://localhost/lists/tuple_string_list', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {string[]} x 
* @returns {Promise<string[]>} 
*/
            export async function stringList (x) {
                return fetch('ipc://localhost/lists/string_list', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {SomeRecord[]} x 
* @returns {Promise<OtherRecord[]>} 
*/
            export async function recordList (x) {
                return fetch('ipc://localhost/lists/record_list', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {OtherRecord[]} x 
* @returns {Promise<SomeRecord[]>} 
*/
            export async function recordListReverse (x) {
                return fetch('ipc://localhost/lists/record_list_reverse', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {SomeVariant[]} x 
* @returns {Promise<OtherVariant[]>} 
*/
            export async function variantList (x) {
                return fetch('ipc://localhost/lists/variant_list', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            /**
* @param {LoadStoreAllSizes} a 
* @returns {Promise<LoadStoreAllSizes>} 
*/
            export async function loadStoreEverything (a) {
                return fetch('ipc://localhost/lists/load_store_everything', { method: "POST", body: JSON.stringify([a]) }).then(r => r.json())
            }
        
