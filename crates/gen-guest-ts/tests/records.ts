

export interface Empty {  }
/**
 * A record containing two scalar fields 
 * that both have the same type 
*/
export interface Scalars { /**
 * The first field, named a 
*/
a: number,
/**
 * The second field, named b 
*/
b: number,
 }
/**
 * A record that is really just flags 
 * All of the fields are bool 
*/
export interface ReallyFlags { 
a: boolean,

b: boolean,

c: boolean,

d: boolean,

e: boolean,

f: boolean,

g: boolean,

h: boolean,

i: boolean,
 }

export interface Aggregates { 
a: Scalars,

b: number,

c: Empty,

d: string,

e: ReallyFlags,
 }

export type IntTypedef = number;

export type TupleTypedef2 = [IntTypedef];


            
            export async function tupleArg (x: [string, number])  {
                return fetch('ipc://localhost/records/tuple_arg', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function tupleResult () : Promise<[string, number]> {
                return fetch('ipc://localhost/records/tuple_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function emptyArg (x: Empty)  {
                return fetch('ipc://localhost/records/empty_arg', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function emptyResult () : Promise<Empty> {
                return fetch('ipc://localhost/records/empty_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function scalarArg (x: Scalars)  {
                return fetch('ipc://localhost/records/scalar_arg', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function scalarResult () : Promise<Scalars> {
                return fetch('ipc://localhost/records/scalar_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function flagsArg (x: ReallyFlags)  {
                return fetch('ipc://localhost/records/flags_arg', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function flagsResult () : Promise<ReallyFlags> {
                return fetch('ipc://localhost/records/flags_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function aggregateArg (x: Aggregates)  {
                return fetch('ipc://localhost/records/aggregate_arg', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function aggregateResult () : Promise<Aggregates> {
                return fetch('ipc://localhost/records/aggregate_result', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function typedefInout (e: TupleTypedef2) : Promise<number> {
                return fetch('ipc://localhost/records/typedef_inout', { method: "POST", body: JSON.stringify([e]) }).then(r => r.json())
            }
        