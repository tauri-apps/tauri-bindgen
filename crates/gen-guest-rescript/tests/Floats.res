@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
let idlHash = "b2ded0ef970e6596"
let float32Param = (x: float): Promise.t<unit> => {
  invoke(~cmd="plugin:floats|float32_param", ~payload={"idlHash": idlHash, "x": x})
}
let float64Param = (x: float): Promise.t<unit> => {
  invoke(~cmd="plugin:floats|float64_param", ~payload={"idlHash": idlHash, "x": x})
}
let float32Result = (): Promise.t<float> => {
  invoke(~cmd="plugin:floats|float32_result", ~payload={"idlHash": idlHash})
}
let float64Result = (): Promise.t<float> => {
  invoke(~cmd="plugin:floats|float64_result", ~payload={"idlHash": idlHash})
}
