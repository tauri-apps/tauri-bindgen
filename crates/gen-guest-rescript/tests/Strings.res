@scope(("window", "__TAURI__", "tauri"))
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "invoke"
let a = (x: string): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|a", ~payload={"x": x})->ignore
}
let b = (): Promise.t<string> => {
  invoke(~cmd="plugin:imports|b")->ignore
}
let c = (a: string, b: string): Promise.t<string> => {
  invoke(~cmd="plugin:imports|c", ~payload={"a": a, "b": b})->ignore
}
