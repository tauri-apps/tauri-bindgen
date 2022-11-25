@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
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
  invoke(~cmd="plugin:e6872cf01241a6f3|tuple_arg", ~payload={"x": x})
}
let tupleResult = (): Promise.t<(char, int)> => {
  invoke(~cmd="plugin:e6872cf01241a6f3|tuple_result")
}
let emptyArg = (x: empty): Promise.t<unit> => {
  invoke(~cmd="plugin:e6872cf01241a6f3|empty_arg", ~payload={"x": x})
}
let emptyResult = (): Promise.t<empty> => {
  invoke(~cmd="plugin:e6872cf01241a6f3|empty_result")
}
let scalarArg = (x: scalars): Promise.t<unit> => {
  invoke(~cmd="plugin:e6872cf01241a6f3|scalar_arg", ~payload={"x": x})
}
let scalarResult = (): Promise.t<scalars> => {
  invoke(~cmd="plugin:e6872cf01241a6f3|scalar_result")
}
let flagsArg = (x: reallyFlags): Promise.t<unit> => {
  invoke(~cmd="plugin:e6872cf01241a6f3|flags_arg", ~payload={"x": x})
}
let flagsResult = (): Promise.t<reallyFlags> => {
  invoke(~cmd="plugin:e6872cf01241a6f3|flags_result")
}
let aggregateArg = (x: aggregates): Promise.t<unit> => {
  invoke(~cmd="plugin:e6872cf01241a6f3|aggregate_arg", ~payload={"x": x})
}
let aggregateResult = (): Promise.t<aggregates> => {
  invoke(~cmd="plugin:e6872cf01241a6f3|aggregate_result")
}
let typedefInout = (e: tupleTypedef2): Promise.t<int> => {
  invoke(~cmd="plugin:e6872cf01241a6f3|typedef_inout", ~payload={"e": e})
}
