@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
type ludicrousSpeed = {
  howFastAreYouGoing: int,
  iAmGoingExtremelySlow: int64,
}
let kebabCase = (): Promise.t<unit> => {
  invoke(~cmd="plugin:48646a1b|kebab_case")
}
let foo = (x: ludicrousSpeed): Promise.t<unit> => {
  invoke(~cmd="plugin:48646a1b|foo", ~payload={"x": x})
}
let functionWithDashes = (): Promise.t<unit> => {
  invoke(~cmd="plugin:48646a1b|function_with_dashes")
}
let functionWithNoWeirdCharacters = (): Promise.t<unit> => {
  invoke(~cmd="plugin:48646a1b|function_with_no_weird_characters")
}
let apple = (): Promise.t<unit> => {
  invoke(~cmd="plugin:48646a1b|apple")
}
let applePear = (): Promise.t<unit> => {
  invoke(~cmd="plugin:48646a1b|apple_pear")
}
let applePearGrape = (): Promise.t<unit> => {
  invoke(~cmd="plugin:48646a1b|apple_pear_grape")
}
let a0 = (): Promise.t<unit> => {
  invoke(~cmd="plugin:48646a1b|a0")
}
let isXml = (): Promise.t<unit> => {
  invoke(~cmd="plugin:48646a1b|is_xml")
}
let explicit = (): Promise.t<unit> => {
  invoke(~cmd="plugin:48646a1b|explicit")
}
let explicitKebab = (): Promise.t<unit> => {
  invoke(~cmd="plugin:48646a1b|explicit_kebab")
}
let bool = (): Promise.t<unit> => {
  invoke(~cmd="plugin:48646a1b|bool")
}
