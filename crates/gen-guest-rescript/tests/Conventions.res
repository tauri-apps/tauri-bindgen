@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
if Belt.Option.isNone(%external(__TAURI_BINDGEN_VERSION_CHECK__)) {
  invoke(~cmd="plugin:conventions|48646a1b1c089063e7b03a4c1dd9f5ad")->catch(e => {
    Js.Console.error(
      "The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`.",
    )
  })
}
type ludicrousSpeed = {
  howFastAreYouGoing: int,
  iAmGoingExtremelySlow: int64,
}
let kebabCase = (): Promise.t<unit> => {
  invoke(~cmd="plugin:conventions|kebab_case")
}
let foo = (x: ludicrousSpeed): Promise.t<unit> => {
  invoke(~cmd="plugin:conventions|foo", ~payload={"x": x})
}
let functionWithDashes = (): Promise.t<unit> => {
  invoke(~cmd="plugin:conventions|function_with_dashes")
}
let functionWithNoWeirdCharacters = (): Promise.t<unit> => {
  invoke(~cmd="plugin:conventions|function_with_no_weird_characters")
}
let apple = (): Promise.t<unit> => {
  invoke(~cmd="plugin:conventions|apple")
}
let applePear = (): Promise.t<unit> => {
  invoke(~cmd="plugin:conventions|apple_pear")
}
let applePearGrape = (): Promise.t<unit> => {
  invoke(~cmd="plugin:conventions|apple_pear_grape")
}
let a0 = (): Promise.t<unit> => {
  invoke(~cmd="plugin:conventions|a0")
}
let isXml = (): Promise.t<unit> => {
  invoke(~cmd="plugin:conventions|is_xml")
}
let explicit = (): Promise.t<unit> => {
  invoke(~cmd="plugin:conventions|explicit")
}
let explicitKebab = (): Promise.t<unit> => {
  invoke(~cmd="plugin:conventions|explicit_kebab")
}
let bool = (): Promise.t<unit> => {
  invoke(~cmd="plugin:conventions|bool")
}
