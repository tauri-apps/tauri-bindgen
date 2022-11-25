@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
if Belt.Option.isNone(%external(__TAURI_BINDGEN_VERSION_CHECK__)) {
  invoke(~cmd="plugin:floats|b2ded0ef970e65969239249842d626ce")->catch(e => {
    Js.Console.error(
      "The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`.",
    )
  })
}
let float32Param = (x: float): Promise.t<unit> => {
  invoke(~cmd="plugin:floats|float32_param", ~payload={"x": x})
}
let float64Param = (x: float): Promise.t<unit> => {
  invoke(~cmd="plugin:floats|float64_param", ~payload={"x": x})
}
let float32Result = (): Promise.t<float> => {
  invoke(~cmd="plugin:floats|float32_result")
}
let float64Result = (): Promise.t<float> => {
  invoke(~cmd="plugin:floats|float64_result")
}
