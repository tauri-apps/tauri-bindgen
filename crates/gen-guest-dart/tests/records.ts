

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
            }
        
            
            export async function tupleResult () : Promise<[string, number]> {
            }
        
            
            export async function emptyArg (x: Empty)  {
            }
        
            
            export async function emptyResult () : Promise<Empty> {
            }
        
            
            export async function scalarArg (x: Scalars)  {
            }
        
            
            export async function scalarResult () : Promise<Scalars> {
            }
        
            
            export async function flagsArg (x: ReallyFlags)  {
            }
        
            
            export async function flagsResult () : Promise<ReallyFlags> {
            }
        
            
            export async function aggregateArg (x: Aggregates)  {
            }
        
            
            export async function aggregateResult () : Promise<Aggregates> {
            }
        
            
            export async function typedefInout (e: TupleTypedef2) : Promise<number> {
            }
        