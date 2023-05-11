export type Result<T, E> = { tag: 'ok', val: T } | { tag: 'err', val: E };


export enum Error { 
Success,

Failure,
 }


            
            export async function optionTest () : Promise<Result<string | null, Error>> {
            }
        