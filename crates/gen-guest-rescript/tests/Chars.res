@scope(("window", "__TAURI__", "tauri"))
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "invoke"
/**
* A function that accepts a character
*/
let takeChar = (x: char): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|take_char", ~payload={"x": x})->ignore
}
/**
* A function that returns a character
*/
let returnChar = (): Promise.t<char> => {
  invoke(~cmd="plugin:imports|return_char")->ignore
}
