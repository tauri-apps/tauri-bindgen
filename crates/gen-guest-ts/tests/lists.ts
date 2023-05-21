

export interface OtherRecord { 
a1: number,

a2: bigint,

a3: number,

a4: bigint,

b: string,

c: Uint8Array[],
 }

export interface SomeRecord { 
x: string,

y: OtherRecord,

z: OtherRecord[],

c1: number,

c2: bigint,

c3: number,

c4: bigint,
 }

export interface OtherVariantA { tag: 0 }

export interface OtherVariantB { tag: 1, value: number }

export interface OtherVariantC { tag: 2, value: string }


export type OtherVariant = 
OtherVariantA | 
OtherVariantB | 
OtherVariantC

export interface SomeVariantA { tag: 0, value: string }

export interface SomeVariantB { tag: 1 }

export interface SomeVariantC { tag: 2, value: number }

export interface SomeVariantD { tag: 3, value: OtherVariant[] }


export type SomeVariant = 
SomeVariantA | 
SomeVariantB | 
SomeVariantC | 
SomeVariantD

export type LoadStoreAllSizes = [string, number, number, number, number, number, number, bigint, bigint, number, number, string][];


            
            export async function listU8Param (x: Uint8Array[])  {
                return fetch('ipc://localhost/lists/list_u8_param', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function listU16Param (x: Uint16Array[])  {
                return fetch('ipc://localhost/lists/list_u16_param', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function listU32Param (x: Uint32Array[])  {
                return fetch('ipc://localhost/lists/list_u32_param', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function listU64Param (x: BigUint64Array[])  {
                return fetch('ipc://localhost/lists/list_u64_param', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function listS8Param (x: Int8Array[])  {
                return fetch('ipc://localhost/lists/list_s8_param', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function listS16Param (x: Int16Array[])  {
                return fetch('ipc://localhost/lists/list_s16_param', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function listS32Param (x: Int32Array[])  {
                return fetch('ipc://localhost/lists/list_s32_param', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function listS64Param (x: BigInt64Array[])  {
                return fetch('ipc://localhost/lists/list_s64_param', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function listFloat32Param (x: Float32Array[])  {
                return fetch('ipc://localhost/lists/list_float32_param', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function listFloat64Param (x: Float64Array[])  {
                return fetch('ipc://localhost/lists/list_float64_param', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function listU8Ret () : Promise<Uint8Array[]> {
                return fetch('ipc://localhost/lists/list_u8_ret', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function listU16Ret () : Promise<Uint16Array[]> {
                return fetch('ipc://localhost/lists/list_u16_ret', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function listU32Ret () : Promise<Uint32Array[]> {
                return fetch('ipc://localhost/lists/list_u32_ret', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function listU64Ret () : Promise<BigUint64Array[]> {
                return fetch('ipc://localhost/lists/list_u64_ret', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function listS8Ret () : Promise<Int8Array[]> {
                return fetch('ipc://localhost/lists/list_s8_ret', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function listS16Ret () : Promise<Int16Array[]> {
                return fetch('ipc://localhost/lists/list_s16_ret', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function listS32Ret () : Promise<Int32Array[]> {
                return fetch('ipc://localhost/lists/list_s32_ret', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function listS64Ret () : Promise<BigInt64Array[]> {
                return fetch('ipc://localhost/lists/list_s64_ret', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function listFloat32Ret () : Promise<Float32Array[]> {
                return fetch('ipc://localhost/lists/list_float32_ret', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function listFloat64Ret () : Promise<Float64Array[]> {
                return fetch('ipc://localhost/lists/list_float64_ret', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function tupleList (x: [number, number][]) : Promise<[bigint, number][]> {
                return fetch('ipc://localhost/lists/tuple_list', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function stringListArg (a: string[])  {
                return fetch('ipc://localhost/lists/string_list_arg', { method: "POST", body: JSON.stringify([a]) }).then(r => r.json())
            }
        
            
            export async function stringListRet () : Promise<string[]> {
                return fetch('ipc://localhost/lists/string_list_ret', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function tupleStringList (x: [number, string][]) : Promise<[string, number][]> {
                return fetch('ipc://localhost/lists/tuple_string_list', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function stringList (x: string[]) : Promise<string[]> {
                return fetch('ipc://localhost/lists/string_list', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function recordList (x: SomeRecord[]) : Promise<OtherRecord[]> {
                return fetch('ipc://localhost/lists/record_list', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function recordListReverse (x: OtherRecord[]) : Promise<SomeRecord[]> {
                return fetch('ipc://localhost/lists/record_list_reverse', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function variantList (x: SomeVariant[]) : Promise<OtherVariant[]> {
                return fetch('ipc://localhost/lists/variant_list', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function loadStoreEverything (a: LoadStoreAllSizes) : Promise<LoadStoreAllSizes> {
                return fetch('ipc://localhost/lists/load_store_everything', { method: "POST", body: JSON.stringify([a]) }).then(r => r.json())
            }
        