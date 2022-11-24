@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
if Belt.Option.isNone(%external(__TAURI_BINDGEN_VERSION_CHECK__)) {
  invoke(~cmd="plugin:chars|678374cfb5cdb2b5ba845e4b559f402a")->catch(e => {
    Js.Console.error(
      "The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`.",
    )
  })
}
/**
* A function that accepts a character
*/
let takeChar = (x: char): Promise.t<unit> => {
  invoke(~cmd="plugin:chars|take_char", ~payload={"x": x})
}
/**
* A function that returns a character
*/
let returnChar = (): Promise.t<char> => {
  invoke(~cmd="plugin:chars|return_char")
}
