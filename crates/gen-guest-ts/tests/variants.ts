export type Result<T, E> = { tag: 'ok', val: T } | { tag: 'err', val: E };


export enum E1 { 
A,
 }

export type U1 = 
number
 | 
number
;

export interface Empty {  }

export interface V1A { tag: 0 }

export interface V1B { tag: 1, value: U1 }

export interface V1C { tag: 2, value: E1 }

export interface V1D { tag: 3, value: string }

export interface V1E { tag: 4, value: Empty }

export interface V1F { tag: 5 }

export interface V1G { tag: 6, value: number }


export type V1 = 
V1A | 
V1B | 
V1C | 
V1D | 
V1E | 
V1F | 
V1G

export interface Casts1A { tag: 0, value: number }

export interface Casts1B { tag: 1, value: number }


export type Casts1 = 
Casts1A | 
Casts1B

export interface Casts2A { tag: 0, value: number }

export interface Casts2B { tag: 1, value: number }


export type Casts2 = 
Casts2A | 
Casts2B

export interface Casts3A { tag: 0, value: number }

export interface Casts3B { tag: 1, value: bigint }


export type Casts3 = 
Casts3A | 
Casts3B

export interface Casts4A { tag: 0, value: number }

export interface Casts4B { tag: 1, value: bigint }


export type Casts4 = 
Casts4A | 
Casts4B

export interface Casts5A { tag: 0, value: number }

export interface Casts5B { tag: 1, value: bigint }


export type Casts5 = 
Casts5A | 
Casts5B

export interface Casts6A { tag: 0, value: [number, number] }

export interface Casts6B { tag: 1, value: [number, number] }


export type Casts6 = 
Casts6A | 
Casts6B

export enum MyErrno { 
Bad1,

Bad2,
 }

export interface IsClone { 
v1: V1,
 }


            
            export async function e1Arg (x: E1)  {
                return fetch('ipc://localhost/variants/e1_arg', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function e1Result () : Promise<E1> {
                return fetch('ipc://localhost/variants/e1_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function u1Arg (x: U1)  {
                return fetch('ipc://localhost/variants/u1_arg', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function u1Result () : Promise<U1> {
                return fetch('ipc://localhost/variants/u1_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function v1Arg (x: V1)  {
                return fetch('ipc://localhost/variants/v1_arg', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function v1Result () : Promise<V1> {
                return fetch('ipc://localhost/variants/v1_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function boolArg (x: boolean)  {
                return fetch('ipc://localhost/variants/bool_arg', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function boolResult () : Promise<boolean> {
                return fetch('ipc://localhost/variants/bool_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function optionArg (a: boolean | null, b: [] | null, c: number | null, d: E1 | null, e: number | null, f: U1 | null, g: boolean | null | null)  {
                return fetch('ipc://localhost/variants/option_arg', { method: "POST", body: JSON.stringify([a, b, c, d, e, f, g]) }).then(r => r.json())
            }
        
            
            export async function optionResult () : Promise<[boolean | null, [] | null, number | null, E1 | null, number | null, U1 | null, boolean | null | null]> {
                return fetch('ipc://localhost/variants/option_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function casts (a: Casts1, b: Casts2, c: Casts3, d: Casts4, e: Casts5, f: Casts6) : Promise<[Casts1, Casts2, Casts3, Casts4, Casts5, Casts6]> {
                return fetch('ipc://localhost/variants/casts', { method: "POST", body: JSON.stringify([a, b, c, d, e, f]) }).then(r => r.json())
            }
        
            
            export async function resultArg (a: Result<_, _>, b: Result<_, E1>, c: Result<E1, _>, d: Result<[], []>, e: Result<number, V1>, f: Result<string, Uint8Array[]>)  {
                return fetch('ipc://localhost/variants/result_arg', { method: "POST", body: JSON.stringify([a, b, c, d, e, f]) }).then(r => r.json())
            }
        
            
            export async function resultResult () : Promise<[Result<_, _>, Result<_, E1>, Result<E1, _>, Result<[], []>, Result<number, V1>, Result<string, Uint8Array[]>]> {
                return fetch('ipc://localhost/variants/result_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function returnResultSugar () : Promise<Result<number, MyErrno>> {
                return fetch('ipc://localhost/variants/return_result_sugar', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function returnResultSugar2 () : Promise<Result<_, MyErrno>> {
                return fetch('ipc://localhost/variants/return_result_sugar2', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function returnResultSugar3 () : Promise<Result<MyErrno, MyErrno>> {
                return fetch('ipc://localhost/variants/return_result_sugar3', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function returnResultSugar4 () : Promise<Result<[number, number], MyErrno>> {
                return fetch('ipc://localhost/variants/return_result_sugar4', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function returnOptionSugar () : Promise<number | null> {
                return fetch('ipc://localhost/variants/return_option_sugar', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function returnOptionSugar2 () : Promise<MyErrno | null> {
                return fetch('ipc://localhost/variants/return_option_sugar2', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function resultSimple () : Promise<Result<number, number>> {
                return fetch('ipc://localhost/variants/result_simple', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function isCloneArg (a: IsClone)  {
                return fetch('ipc://localhost/variants/is_clone_arg', { method: "POST", body: JSON.stringify([a]) }).then(r => r.json())
            }
        
            
            export async function isCloneReturn () : Promise<IsClone> {
                return fetch('ipc://localhost/variants/is_clone_return', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function returnNamedOption () : Promise<number | null> {
                return fetch('ipc://localhost/variants/return_named_option', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function returnNamedResult () : Promise<Result<number, MyErrno>> {
                return fetch('ipc://localhost/variants/return_named_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        