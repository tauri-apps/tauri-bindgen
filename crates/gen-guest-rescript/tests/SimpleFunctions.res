@scope(("window", "__TAURI__", "tauri"))
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "invoke"
let f1 = (): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|f1")->ignore
}
let f2 = (a: int): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|f2", ~payload={"a": a})->ignore
}
let f3 = (a: int, b: int): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|f3", ~payload={"a": a, "b": b})->ignore
}
let f4 = (): Promise.t<int> => {
  invoke(~cmd="plugin:imports|f4")->ignore
}
let f5 = (): Promise.t<(int, int)> => {
  invoke(~cmd="plugin:imports|f5")->ignore
}
let f6 = (a: int, b: int, c: int): Promise.t<(int, int, int)> => {
  invoke(~cmd="plugin:imports|f6", ~payload={"a": a, "b": b, "c": c})->ignore
}
