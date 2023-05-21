
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
                return fetch('ipc://localhost/unions/add_one_integer', { method: "POST", body: JSON.stringify([num]) }).then(r => r.json())
            }
        
            
            export async function addOneFloat (num: AllFloats) : Promise<AllFloats> {
                return fetch('ipc://localhost/unions/add_one_float', { method: "POST", body: JSON.stringify([num]) }).then(r => r.json())
            }
        
            
            export async function replaceFirstChar (text: AllText, letter: string) : Promise<AllText> {
                return fetch('ipc://localhost/unions/replace_first_char', { method: "POST", body: JSON.stringify([text, letter]) }).then(r => r.json())
            }
        
            
            export async function identifyInteger (num: AllIntegers) : Promise<number> {
                return fetch('ipc://localhost/unions/identify_integer', { method: "POST", body: JSON.stringify([num]) }).then(r => r.json())
            }
        
            
            export async function identifyFloat (num: AllFloats) : Promise<number> {
                return fetch('ipc://localhost/unions/identify_float', { method: "POST", body: JSON.stringify([num]) }).then(r => r.json())
            }
        
            
            export async function identifyText (text: AllText) : Promise<number> {
                return fetch('ipc://localhost/unions/identify_text', { method: "POST", body: JSON.stringify([text]) }).then(r => r.json())
            }
        
            
            export async function addOneDuplicated (num: DuplicatedS32) : Promise<DuplicatedS32> {
                return fetch('ipc://localhost/unions/add_one_duplicated', { method: "POST", body: JSON.stringify([num]) }).then(r => r.json())
            }
        
            
            export async function identifyDuplicated (num: DuplicatedS32) : Promise<number> {
                return fetch('ipc://localhost/unions/identify_duplicated', { method: "POST", body: JSON.stringify([num]) }).then(r => r.json())
            }
        
            
            export async function addOneDistinguishableNum (num: DistinguishableNum) : Promise<DistinguishableNum> {
                return fetch('ipc://localhost/unions/add_one_distinguishable_num', { method: "POST", body: JSON.stringify([num]) }).then(r => r.json())
            }
        
            
            export async function identifyDistinguishableNum (num: DistinguishableNum) : Promise<number> {
                return fetch('ipc://localhost/unions/identify_distinguishable_num', { method: "POST", body: JSON.stringify([num]) }).then(r => r.json())
            }
        