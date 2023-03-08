

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


            
            export async function listU8Param (x: Uint8Array[]) : Promise<[]> {
            }
        
            
            export async function listU16Param (x: Uint16Array[]) : Promise<[]> {
            }
        
            
            export async function listU32Param (x: Uint32Array[]) : Promise<[]> {
            }
        
            
            export async function listU64Param (x: BigUint64Array[]) : Promise<[]> {
            }
        
            
            export async function listS8Param (x: Int8Array[]) : Promise<[]> {
            }
        
            
            export async function listS16Param (x: Int16Array[]) : Promise<[]> {
            }
        
            
            export async function listS32Param (x: Int32Array[]) : Promise<[]> {
            }
        
            
            export async function listS64Param (x: BigInt64Array[]) : Promise<[]> {
            }
        
            
            export async function listFloat32Param (x: Float32Array[]) : Promise<[]> {
            }
        
            
            export async function listFloat64Param (x: Float64Array[]) : Promise<[]> {
            }
        
            
            export async function listU8Ret () : Promise<Uint8Array[]> {
            }
        
            
            export async function listU16Ret () : Promise<Uint16Array[]> {
            }
        
            
            export async function listU32Ret () : Promise<Uint32Array[]> {
            }
        
            
            export async function listU64Ret () : Promise<BigUint64Array[]> {
            }
        
            
            export async function listS8Ret () : Promise<Int8Array[]> {
            }
        
            
            export async function listS16Ret () : Promise<Int16Array[]> {
            }
        
            
            export async function listS32Ret () : Promise<Int32Array[]> {
            }
        
            
            export async function listS64Ret () : Promise<BigInt64Array[]> {
            }
        
            
            export async function listFloat32Ret () : Promise<Float32Array[]> {
            }
        
            
            export async function listFloat64Ret () : Promise<Float64Array[]> {
            }
        
            
            export async function tupleList (x: [number, number][]) : Promise<[bigint, number][]> {
            }
        
            
            export async function stringListArg (a: string[]) : Promise<[]> {
            }
        
            
            export async function stringListRet () : Promise<string[]> {
            }
        
            
            export async function tupleStringList (x: [number, string][]) : Promise<[string, number][]> {
            }
        
            
            export async function stringList (x: string[]) : Promise<string[]> {
            }
        
            
            export async function recordList (x: SomeRecord[]) : Promise<OtherRecord[]> {
            }
        
            
            export async function recordListReverse (x: OtherRecord[]) : Promise<SomeRecord[]> {
            }
        
            
            export async function variantList (x: SomeVariant[]) : Promise<OtherVariant[]> {
            }
        
            
            export async function loadStoreEverything (a: LoadStoreAllSizes) : Promise<LoadStoreAllSizes> {
            }
        