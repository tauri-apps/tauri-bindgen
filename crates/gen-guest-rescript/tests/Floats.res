@scope(("window", "__TAURI__", "tauri"))
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "invoke"
let float32Param = (x: float): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|float32_param", ~payload={"x": x})->ignore
}
let float64Param = (x: float): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|float64_param", ~payload={"x": x})->ignore
}
let float32Result = (): Promise.t<float> => {
  invoke(~cmd="plugin:imports|float32_result")->ignore
}
let float64Result = (): Promise.t<float> => {
  invoke(~cmd="plugin:imports|float64_result")->ignore
}
