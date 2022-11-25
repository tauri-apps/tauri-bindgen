@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
type ludicrousSpeed = {
  howFastAreYouGoing: int,
  iAmGoingExtremelySlow: int64,
}
let kebabCase = (): Promise.t<unit> => {
  invoke(~cmd="plugin:48646a1b1c089063|kebab_case")
}
let foo = (x: ludicrousSpeed): Promise.t<unit> => {
  invoke(~cmd="plugin:48646a1b1c089063|foo", ~payload={"x": x})
}
let functionWithDashes = (): Promise.t<unit> => {
  invoke(~cmd="plugin:48646a1b1c089063|function_with_dashes")
}
let functionWithNoWeirdCharacters = (): Promise.t<unit> => {
  invoke(~cmd="plugin:48646a1b1c089063|function_with_no_weird_characters")
}
let apple = (): Promise.t<unit> => {
  invoke(~cmd="plugin:48646a1b1c089063|apple")
}
let applePear = (): Promise.t<unit> => {
  invoke(~cmd="plugin:48646a1b1c089063|apple_pear")
}
let applePearGrape = (): Promise.t<unit> => {
  invoke(~cmd="plugin:48646a1b1c089063|apple_pear_grape")
}
let a0 = (): Promise.t<unit> => {
  invoke(~cmd="plugin:48646a1b1c089063|a0")
}
let isXml = (): Promise.t<unit> => {
  invoke(~cmd="plugin:48646a1b1c089063|is_xml")
}
let explicit = (): Promise.t<unit> => {
  invoke(~cmd="plugin:48646a1b1c089063|explicit")
}
let explicitKebab = (): Promise.t<unit> => {
  invoke(~cmd="plugin:48646a1b1c089063|explicit_kebab")
}
let bool = (): Promise.t<unit> => {
  invoke(~cmd="plugin:48646a1b1c089063|bool")
}
