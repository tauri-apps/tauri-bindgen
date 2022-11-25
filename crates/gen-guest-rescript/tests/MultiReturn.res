@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
if Belt.Option.isNone(%external(__TAURI_BINDGEN_VERSION_CHECK__)) {
  invoke(~cmd="plugin:multi_return|d238f57052cdcb9073d14f7a8059345b")->catch(e => {
    Js.Console.error(
      "The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`.",
    )
  })
}
let mra = (): Promise.t<unit> => {
  invoke(~cmd="plugin:multi_return|mra")
}
let mrb = (): Promise.t<unit> => {
  invoke(~cmd="plugin:multi_return|mrb")
}
let mrc = (): Promise.t<int> => {
  invoke(~cmd="plugin:multi_return|mrc")
}
let mrd = (): Promise.t<int> => {
  invoke(~cmd="plugin:multi_return|mrd")
}
let mre = (): Promise.t<(int, float)> => {
  invoke(~cmd="plugin:multi_return|mre")
}
