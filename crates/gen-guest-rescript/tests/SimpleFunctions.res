@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
if Belt.Option.isNone(%external(__TAURI_BINDGEN_VERSION_CHECK__)) {
  invoke(~cmd="plugin:simple|ebb2d6f0441e00a02915e2faf10bbe90")->catch(e => {
    Js.Console.error(
      "The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`.",
    )
  })
}
let f1 = (): Promise.t<unit> => {
  invoke(~cmd="plugin:simple|f1")
}
let f2 = (a: int): Promise.t<unit> => {
  invoke(~cmd="plugin:simple|f2", ~payload={"a": a})
}
let f3 = (a: int, b: int): Promise.t<unit> => {
  invoke(~cmd="plugin:simple|f3", ~payload={"a": a, "b": b})
}
let f4 = (): Promise.t<int> => {
  invoke(~cmd="plugin:simple|f4")
}
let f5 = (): Promise.t<(int, int)> => {
  invoke(~cmd="plugin:simple|f5")
}
let f6 = (a: int, b: int, c: int): Promise.t<(int, int, int)> => {
  invoke(~cmd="plugin:simple|f6", ~payload={"a": a, "b": b, "c": c})
}
