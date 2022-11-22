@scope(("window", "__TAURI__", "tauri"))
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "invoke"
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
  invoke(~cmd="plugin:imports|tuple_arg", ~payload={"x": x})->ignore
}
let tupleResult = (): Promise.t<(char, int)> => {
  invoke(~cmd="plugin:imports|tuple_result")->ignore
}
let emptyArg = (x: empty): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|empty_arg", ~payload={"x": x})->ignore
}
let emptyResult = (): Promise.t<empty> => {
  invoke(~cmd="plugin:imports|empty_result")->ignore
}
let scalarArg = (x: scalars): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|scalar_arg", ~payload={"x": x})->ignore
}
let scalarResult = (): Promise.t<scalars> => {
  invoke(~cmd="plugin:imports|scalar_result")->ignore
}
let flagsArg = (x: reallyFlags): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|flags_arg", ~payload={"x": x})->ignore
}
let flagsResult = (): Promise.t<reallyFlags> => {
  invoke(~cmd="plugin:imports|flags_result")->ignore
}
let aggregateArg = (x: aggregates): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|aggregate_arg", ~payload={"x": x})->ignore
}
let aggregateResult = (): Promise.t<aggregates> => {
  invoke(~cmd="plugin:imports|aggregate_result")->ignore
}
let typedefInout = (e: tupleTypedef2): Promise.t<int> => {
  invoke(~cmd="plugin:imports|typedef_inout", ~payload={"e": e})->ignore
}
