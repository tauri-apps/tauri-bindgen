@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
let idlHash = "678374cfb5cdb2b5"
/**
* A function that accepts a character
*/
let takeChar = (x: char): Promise.t<unit> => {
  invoke(~cmd="plugin:chars|take_char", ~payload={"idlHash": idlHash, "x": x})
}
/**
* A function that returns a character
*/
let returnChar = (): Promise.t<char> => {
  invoke(~cmd="plugin:chars|return_char", ~payload={"idlHash": idlHash})
}
