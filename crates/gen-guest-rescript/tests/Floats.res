@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
let float32Param = (x: float): Promise.t<unit> => {
  invoke(~cmd="plugin:b2ded0ef970e6596|float32_param", ~payload={"x": x})
}
let float64Param = (x: float): Promise.t<unit> => {
  invoke(~cmd="plugin:b2ded0ef970e6596|float64_param", ~payload={"x": x})
}
let float32Result = (): Promise.t<float> => {
  invoke(~cmd="plugin:b2ded0ef970e6596|float32_result")
}
let float64Result = (): Promise.t<float> => {
  invoke(~cmd="plugin:b2ded0ef970e6596|float64_result")
}
