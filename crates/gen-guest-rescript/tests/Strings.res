@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
if Belt.Option.isNone(%external(__TAURI_BINDGEN_VERSION_CHECK__)) {
  invoke(~cmd="plugin:strings|16c3ebd2deefea81065e2001501951a6")->catch(e => {
    Js.Console.error(
      "The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`.",
    )
  })
}
let a = (x: string): Promise.t<unit> => {
  invoke(~cmd="plugin:strings|a", ~payload={"x": x})
}
let b = (): Promise.t<string> => {
  invoke(~cmd="plugin:strings|b")
}
let c = (a: string, b: string): Promise.t<string> => {
  invoke(~cmd="plugin:strings|c", ~payload={"a": a, "b": b})
}
