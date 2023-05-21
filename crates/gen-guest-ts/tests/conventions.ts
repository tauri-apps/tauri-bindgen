

export interface LudicrousSpeed { 
howFastAreYouGoing: number,

iAmGoingExtremelySlow: bigint,
 }


            
            export async function kebabCase ()  {
                return fetch('ipc://localhost/conventions/kebab_case', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function foo (x: LudicrousSpeed)  {
                return fetch('ipc://localhost/conventions/foo', { method: "POST", body: JSON.stringify([x]) }).then(r => r.json())
            }
        
            
            export async function functionWithUnderscores ()  {
                return fetch('ipc://localhost/conventions/function_with_underscores', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function functionWithNoWeirdCharacters ()  {
                return fetch('ipc://localhost/conventions/function_with_no_weird_characters', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function apple ()  {
                return fetch('ipc://localhost/conventions/apple', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function applePear ()  {
                return fetch('ipc://localhost/conventions/apple_pear', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function applePearGrape ()  {
                return fetch('ipc://localhost/conventions/apple_pear_grape', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function a0 ()  {
                return fetch('ipc://localhost/conventions/a0', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function isXml ()  {
                return fetch('ipc://localhost/conventions/is_xml', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function explicit ()  {
                return fetch('ipc://localhost/conventions/explicit', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function explicitSnake ()  {
                return fetch('ipc://localhost/conventions/explicit_snake', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        
            
            export async function bool ()  {
                return fetch('ipc://localhost/conventions/bool', { method: "POST", body: JSON.stringify([]) }).then(r => r.json())
            }
        