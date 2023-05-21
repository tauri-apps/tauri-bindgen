export class Deserializer {
        source
        offset
    
        constructor(bytes) {
            this.source = bytes
            this.offset = 0
        }
    
        pop() {
            return this.source[this.offset++]
        }
    
        try_take_n(len) {
            const out = this.source.slice(this.offset, this.offset + len)
            this.offset += len
            return out
        }
    }
    

            /**
*/
            export async function kebabCase () {
                return fetch('ipc://localhost/conventions/kebab_case', { method: "POST", body: JSON.stringify([]) })
            }
        
            /**
* @param {LudicrousSpeed} x 
*/
            export async function foo (x) {
                return fetch('ipc://localhost/conventions/foo', { method: "POST", body: JSON.stringify([x]) })
            }
        
            /**
*/
            export async function functionWithUnderscores () {
                return fetch('ipc://localhost/conventions/function_with_underscores', { method: "POST", body: JSON.stringify([]) })
            }
        
            /**
*/
            export async function functionWithNoWeirdCharacters () {
                return fetch('ipc://localhost/conventions/function_with_no_weird_characters', { method: "POST", body: JSON.stringify([]) })
            }
        
            /**
*/
            export async function apple () {
                return fetch('ipc://localhost/conventions/apple', { method: "POST", body: JSON.stringify([]) })
            }
        
            /**
*/
            export async function applePear () {
                return fetch('ipc://localhost/conventions/apple_pear', { method: "POST", body: JSON.stringify([]) })
            }
        
            /**
*/
            export async function applePearGrape () {
                return fetch('ipc://localhost/conventions/apple_pear_grape', { method: "POST", body: JSON.stringify([]) })
            }
        
            /**
*/
            export async function a0 () {
                return fetch('ipc://localhost/conventions/a0', { method: "POST", body: JSON.stringify([]) })
            }
        
            /**
*/
            export async function isXml () {
                return fetch('ipc://localhost/conventions/is_xml', { method: "POST", body: JSON.stringify([]) })
            }
        
            /**
*/
            export async function explicit () {
                return fetch('ipc://localhost/conventions/explicit', { method: "POST", body: JSON.stringify([]) })
            }
        
            /**
*/
            export async function explicitSnake () {
                return fetch('ipc://localhost/conventions/explicit_snake', { method: "POST", body: JSON.stringify([]) })
            }
        
            /**
*/
            export async function bool () {
                return fetch('ipc://localhost/conventions/bool', { method: "POST", body: JSON.stringify([]) })
            }
        
