@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
if Belt.Option.isNone(%external(__TAURI_BINDGEN_VERSION_CHECK__)) {
  invoke(~cmd="plugin:records|e6872cf01241a6f3e4c4bedaa609dbeb")->catch(e => {
    Js.Console.error(
      "The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`.",
    )
  })
}
type empty = unit
/**
* A record containing two scalar fields
* that both have the same type
*/
type scalars = {
  /**
  * The first field, named a
  */
  a: int,
  /**
  * The second field, named b
  */
  b: int,
}
/**
* A record that is really just flags
* All of the fields are bool
*/
type reallyFlags = {
  a: bool,
  b: bool,
  c: bool,
  d: bool,
  e: bool,
  f: bool,
  g: bool,
  h: bool,
  i: bool,
}
type aggregates = {
  a: scalars,
  b: int,
  c: empty,
  d: string,
  e: reallyFlags,
}
type tupleTypedef = int
type intTypedef = int
type tupleTypedef2 = intTypedef
let tupleArg = (x: (char, int)): Promise.t<unit> => {
  invoke(~cmd="plugin:records|tuple_arg", ~payload={"x": x})
}
let tupleResult = (): Promise.t<(char, int)> => {
  invoke(~cmd="plugin:records|tuple_result")
}
let emptyArg = (x: empty): Promise.t<unit> => {
  invoke(~cmd="plugin:records|empty_arg", ~payload={"x": x})
}
let emptyResult = (): Promise.t<empty> => {
  invoke(~cmd="plugin:records|empty_result")
}
let scalarArg = (x: scalars): Promise.t<unit> => {
  invoke(~cmd="plugin:records|scalar_arg", ~payload={"x": x})
}
let scalarResult = (): Promise.t<scalars> => {
  invoke(~cmd="plugin:records|scalar_result")
}
let flagsArg = (x: reallyFlags): Promise.t<unit> => {
  invoke(~cmd="plugin:records|flags_arg", ~payload={"x": x})
}
let flagsResult = (): Promise.t<reallyFlags> => {
  invoke(~cmd="plugin:records|flags_result")
}
let aggregateArg = (x: aggregates): Promise.t<unit> => {
  invoke(~cmd="plugin:records|aggregate_arg", ~payload={"x": x})
}
let aggregateResult = (): Promise.t<aggregates> => {
  invoke(~cmd="plugin:records|aggregate_result")
}
let typedefInout = (e: tupleTypedef2): Promise.t<int> => {
  invoke(~cmd="plugin:records|typedef_inout", ~payload={"e": e})
}
