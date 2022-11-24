@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
/**
* A function that accepts a character
*/
let takeChar = (x: char): Promise.t<unit> => {
  invoke(~cmd="plugin:chars|0", ~payload={"0": x})
}
/**
* A function that returns a character
*/
let returnChar = (): Promise.t<char> => {
  invoke(~cmd="plugin:chars|1")
}
