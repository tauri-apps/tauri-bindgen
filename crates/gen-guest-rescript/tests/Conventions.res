@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
let idlHash = "48646a1b1c089063"
type ludicrousSpeed = {
  howFastAreYouGoing: int,
  iAmGoingExtremelySlow: int64,
}
let kebabCase = (): Promise.t<unit> => {
  invoke(~cmd="plugin:conventions|kebab_case", ~payload={"idlHash": idlHash})
}
let foo = (x: ludicrousSpeed): Promise.t<unit> => {
  invoke(~cmd="plugin:conventions|foo", ~payload={"idlHash": idlHash, "x": x})
}
let functionWithDashes = (): Promise.t<unit> => {
  invoke(~cmd="plugin:conventions|function_with_dashes", ~payload={"idlHash": idlHash})
}
let functionWithNoWeirdCharacters = (): Promise.t<unit> => {
  invoke(~cmd="plugin:conventions|function_with_no_weird_characters", ~payload={"idlHash": idlHash})
}
let apple = (): Promise.t<unit> => {
  invoke(~cmd="plugin:conventions|apple", ~payload={"idlHash": idlHash})
}
let applePear = (): Promise.t<unit> => {
  invoke(~cmd="plugin:conventions|apple_pear", ~payload={"idlHash": idlHash})
}
let applePearGrape = (): Promise.t<unit> => {
  invoke(~cmd="plugin:conventions|apple_pear_grape", ~payload={"idlHash": idlHash})
}
let a0 = (): Promise.t<unit> => {
  invoke(~cmd="plugin:conventions|a0", ~payload={"idlHash": idlHash})
}
let isXml = (): Promise.t<unit> => {
  invoke(~cmd="plugin:conventions|is_xml", ~payload={"idlHash": idlHash})
}
let explicit = (): Promise.t<unit> => {
  invoke(~cmd="plugin:conventions|explicit", ~payload={"idlHash": idlHash})
}
let explicitKebab = (): Promise.t<unit> => {
  invoke(~cmd="plugin:conventions|explicit_kebab", ~payload={"idlHash": idlHash})
}
let bool = (): Promise.t<unit> => {
  invoke(~cmd="plugin:conventions|bool", ~payload={"idlHash": idlHash})
}
