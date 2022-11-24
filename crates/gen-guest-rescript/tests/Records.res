@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
let idlHash = "e6872cf01241a6f3"
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
  invoke(~cmd="plugin:records|tuple_arg", ~payload={"idlHash": idlHash, "x": x})
}
let tupleResult = (): Promise.t<(char, int)> => {
  invoke(~cmd="plugin:records|tuple_result", ~payload={"idlHash": idlHash})
}
let emptyArg = (x: empty): Promise.t<unit> => {
  invoke(~cmd="plugin:records|empty_arg", ~payload={"idlHash": idlHash, "x": x})
}
let emptyResult = (): Promise.t<empty> => {
  invoke(~cmd="plugin:records|empty_result", ~payload={"idlHash": idlHash})
}
let scalarArg = (x: scalars): Promise.t<unit> => {
  invoke(~cmd="plugin:records|scalar_arg", ~payload={"idlHash": idlHash, "x": x})
}
let scalarResult = (): Promise.t<scalars> => {
  invoke(~cmd="plugin:records|scalar_result", ~payload={"idlHash": idlHash})
}
let flagsArg = (x: reallyFlags): Promise.t<unit> => {
  invoke(~cmd="plugin:records|flags_arg", ~payload={"idlHash": idlHash, "x": x})
}
let flagsResult = (): Promise.t<reallyFlags> => {
  invoke(~cmd="plugin:records|flags_result", ~payload={"idlHash": idlHash})
}
let aggregateArg = (x: aggregates): Promise.t<unit> => {
  invoke(~cmd="plugin:records|aggregate_arg", ~payload={"idlHash": idlHash, "x": x})
}
let aggregateResult = (): Promise.t<aggregates> => {
  invoke(~cmd="plugin:records|aggregate_result", ~payload={"idlHash": idlHash})
}
let typedefInout = (e: tupleTypedef2): Promise.t<int> => {
  invoke(~cmd="plugin:records|typedef_inout", ~payload={"idlHash": idlHash, "e": e})
}
