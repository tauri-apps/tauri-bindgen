@scope(("window", "__TAURI__", "tauri"))
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "invoke"
type ludicrousSpeed = {
  howFastAreYouGoing: int,
  iAmGoingExtremelySlow: int64,
}
let kebabCase = (): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|kebab_case")->ignore
}
let foo = (x: ludicrousSpeed): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|foo", ~payload={"x": x})->ignore
}
let functionWithDashes = (): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|function_with_dashes")->ignore
}
let functionWithNoWeirdCharacters = (): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|function_with_no_weird_characters")->ignore
}
let apple = (): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|apple")->ignore
}
let applePear = (): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|apple_pear")->ignore
}
let applePearGrape = (): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|apple_pear_grape")->ignore
}
let a0 = (): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|a0")->ignore
}
let isXml = (): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|is_xml")->ignore
}
let explicit = (): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|explicit")->ignore
}
let explicitKebab = (): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|explicit_kebab")->ignore
}
let bool = (): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|bool")->ignore
}
