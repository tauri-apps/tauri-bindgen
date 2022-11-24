@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
let idlHash = "92d5120c899c41cc"
type bigStruct = {
  a1: string,
  a2: string,
  a3: string,
  a4: string,
  a5: string,
  a6: string,
  a7: string,
  a8: string,
  a9: string,
  a10: string,
  a11: string,
  a12: string,
  a13: string,
  a14: string,
  a15: string,
  a16: string,
  a17: string,
  a18: string,
  a19: string,
  a20: string,
}
let manyArgs = (
  a1: int64,
  a2: int64,
  a3: int64,
  a4: int64,
  a5: int64,
  a6: int64,
  a7: int64,
  a8: int64,
  a9: int64,
  a10: int64,
  a11: int64,
  a12: int64,
  a13: int64,
  a14: int64,
  a15: int64,
  a16: int64,
): Promise.t<unit> => {
  invoke(
    ~cmd="plugin:manyarg|many_args",
    ~payload={
      "idlHash": idlHash,
      "a1": a1,
      "a2": a2,
      "a3": a3,
      "a4": a4,
      "a5": a5,
      "a6": a6,
      "a7": a7,
      "a8": a8,
      "a9": a9,
      "a10": a10,
      "a11": a11,
      "a12": a12,
      "a13": a13,
      "a14": a14,
      "a15": a15,
      "a16": a16,
    },
  )
}
let bigArgument = (x: bigStruct): Promise.t<unit> => {
  invoke(~cmd="plugin:manyarg|big_argument", ~payload={"idlHash": idlHash, "x": x})
}
