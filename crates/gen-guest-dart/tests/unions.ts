
/**
 * A union of all of the integral types 
*/
export type AllIntegers = /**
 * Bool is equivalent to a 1 bit integer 
 * and is treated that way in some languages 
*/
boolean
 | 
number
 | 
number
 | 
number
 | 
bigint
 | 
number
 | 
number
 | 
number
 | 
bigint
;

export type AllFloats = 
number
 | 
number
;

export type AllText = 
string
 | 
string
;

export type DuplicatedS32 = /**
 * The first s32 
*/
number
 | /**
 * The second s32 
*/
number
 | /**
 * The third s32 
*/
number
;
/**
 * A type containing numeric types that are distinct in most languages 
*/
export type DistinguishableNum = /**
 * A Floating Point Number 
*/
number
 | /**
 * A Signed Integer 
*/
bigint
;


            
            export async function addOneInteger (num: AllIntegers) : Promise<AllIntegers> {
            }
        
            
            export async function addOneFloat (num: AllFloats) : Promise<AllFloats> {
            }
        
            
            export async function replaceFirstChar (text: AllText, letter: string) : Promise<AllText> {
            }
        
            
            export async function identifyInteger (num: AllIntegers) : Promise<number> {
            }
        
            
            export async function identifyFloat (num: AllFloats) : Promise<number> {
            }
        
            
            export async function identifyText (text: AllText) : Promise<number> {
            }
        
            
            export async function addOneDuplicated (num: DuplicatedS32) : Promise<DuplicatedS32> {
            }
        
            
            export async function identifyDuplicated (num: DuplicatedS32) : Promise<number> {
            }
        
            
            export async function addOneDistinguishableNum (num: DistinguishableNum) : Promise<DistinguishableNum> {
            }
        
            
            export async function identifyDistinguishableNum (num: DistinguishableNum) : Promise<number> {
            }
        