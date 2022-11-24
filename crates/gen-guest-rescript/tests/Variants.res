@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
let idlHash = "d5901a6520084a85"
type e1 = A

type u1 =
  | U32(int)
  | F32(float)

type empty = unit
type v1 =
  | A
  | B(u1)
  | C(e1)
  | D(string)
  | E(empty)
  | F
  | G(int)

type casts1 =
  | A(int)
  | B(float)

type casts2 =
  | A(float)
  | B(float)

type casts3 =
  | A(float)
  | B(int64)

type casts4 =
  | A(int)
  | B(int64)

type casts5 =
  | A(float)
  | B(int64)

type casts6 =
  | A((float, int))
  | B((int, int))

type myErrno =
  | Bad1
  | Bad2

type isClone = {v1: v1}
let e1Arg = (x: e1): Promise.t<unit> => {
  invoke(~cmd="plugin:variants|e1_arg", ~payload={"idlHash": idlHash, "x": x})
}
let e1Result = (): Promise.t<e1> => {
  invoke(~cmd="plugin:variants|e1_result", ~payload={"idlHash": idlHash})
}
let u1Arg = (x: u1): Promise.t<unit> => {
  invoke(~cmd="plugin:variants|u1_arg", ~payload={"idlHash": idlHash, "x": x})
}
let u1Result = (): Promise.t<u1> => {
  invoke(~cmd="plugin:variants|u1_result", ~payload={"idlHash": idlHash})
}
let v1Arg = (x: v1): Promise.t<unit> => {
  invoke(~cmd="plugin:variants|v1_arg", ~payload={"idlHash": idlHash, "x": x})
}
let v1Result = (): Promise.t<v1> => {
  invoke(~cmd="plugin:variants|v1_result", ~payload={"idlHash": idlHash})
}
let boolArg = (x: bool): Promise.t<unit> => {
  invoke(~cmd="plugin:variants|bool_arg", ~payload={"idlHash": idlHash, "x": x})
}
let boolResult = (): Promise.t<bool> => {
  invoke(~cmd="plugin:variants|bool_result", ~payload={"idlHash": idlHash})
}
let optionArg = (
  a: option<bool>,
  b: option<unit>,
  c: option<int>,
  d: option<e1>,
  e: option<float>,
  f: option<u1>,
  g: option<option<bool>>,
): Promise.t<unit> => {
  invoke(
    ~cmd="plugin:variants|option_arg",
    ~payload={"idlHash": idlHash, "a": a, "b": b, "c": c, "d": d, "e": e, "f": f, "g": g},
  )
}
let optionResult = (): Promise.t<(
  option<bool>,
  option<unit>,
  option<int>,
  option<e1>,
  option<float>,
  option<u1>,
  option<option<bool>>,
)> => {
  invoke(~cmd="plugin:variants|option_result", ~payload={"idlHash": idlHash})
}
let casts = (a: casts1, b: casts2, c: casts3, d: casts4, e: casts5, f: casts6): Promise.t<(
  casts1,
  casts2,
  casts3,
  casts4,
  casts5,
  casts6,
)> => {
  invoke(
    ~cmd="plugin:variants|casts",
    ~payload={"idlHash": idlHash, "a": a, "b": b, "c": c, "d": d, "e": e, "f": f},
  )
}
let resultArg = (
  a: Result.t<unit, unit>,
  b: Result.t<unit, e1>,
  c: Result.t<e1, unit>,
  d: Result.t<unit, unit>,
  e: Result.t<int, v1>,
  f: Result.t<string, TypedArray.uint8Array>,
): Promise.t<unit> => {
  invoke(
    ~cmd="plugin:variants|result_arg",
    ~payload={"idlHash": idlHash, "a": a, "b": b, "c": c, "d": d, "e": e, "f": f},
  )
}
let resultResult = (): Promise.t<(
  Result.t<unit, unit>,
  Result.t<unit, e1>,
  Result.t<e1, unit>,
  Result.t<unit, unit>,
  Result.t<int, v1>,
  Result.t<string, TypedArray.uint8Array>,
)> => {
  invoke(~cmd="plugin:variants|result_result", ~payload={"idlHash": idlHash})
}
let returnResultSugar = (): Promise.t<int> => {
  invoke(~cmd="plugin:variants|return_result_sugar", ~payload={"idlHash": idlHash})
}
let returnResultSugar2 = (): Promise.t<unit> => {
  invoke(~cmd="plugin:variants|return_result_sugar2", ~payload={"idlHash": idlHash})
}
let returnResultSugar3 = (): Promise.t<myErrno> => {
  invoke(~cmd="plugin:variants|return_result_sugar3", ~payload={"idlHash": idlHash})
}
let returnResultSugar4 = (): Promise.t<(int, int)> => {
  invoke(~cmd="plugin:variants|return_result_sugar4", ~payload={"idlHash": idlHash})
}
let returnOptionSugar = (): Promise.t<option<int>> => {
  invoke(~cmd="plugin:variants|return_option_sugar", ~payload={"idlHash": idlHash})
}
let returnOptionSugar2 = (): Promise.t<option<myErrno>> => {
  invoke(~cmd="plugin:variants|return_option_sugar2", ~payload={"idlHash": idlHash})
}
let resultSimple = (): Promise.t<int> => {
  invoke(~cmd="plugin:variants|result_simple", ~payload={"idlHash": idlHash})
}
let isCloneArg = (a: isClone): Promise.t<unit> => {
  invoke(~cmd="plugin:variants|is_clone_arg", ~payload={"idlHash": idlHash, "a": a})
}
let isCloneReturn = (): Promise.t<isClone> => {
  invoke(~cmd="plugin:variants|is_clone_return", ~payload={"idlHash": idlHash})
}
let returnNamedOption = (): Promise.t<option<int>> => {
  invoke(~cmd="plugin:variants|return_named_option", ~payload={"idlHash": idlHash})
}
let returnNamedResult = (): Promise.t<int> => {
  invoke(~cmd="plugin:variants|return_named_result", ~payload={"idlHash": idlHash})
}
