@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
if Belt.Option.isNone(%external(__TAURI_BINDGEN_VERSION_CHECK__)) {
  invoke(~cmd="plugin:simple_lists|d40a3203ef48115d7df3e6859a69ed77")->catch(e => {
    Js.Console.error(
      "The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`.",
    )
  })
}
let simpleList1 = (l: TypedArray.uint32Array): Promise.t<unit> => {
  invoke(~cmd="plugin:simple_lists|simple_list1", ~payload={"l": l})
}
let simpleList2 = (): Promise.t<TypedArray.uint32Array> => {
  invoke(~cmd="plugin:simple_lists|simple_list2")
}
let simpleList4 = (l: array<TypedArray.uint32Array>): Promise.t<array<TypedArray.uint32Array>> => {
  invoke(~cmd="plugin:simple_lists|simple_list4", ~payload={"l": l})
}
