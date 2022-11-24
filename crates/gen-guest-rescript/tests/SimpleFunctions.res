@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
let idlHash = "ebb2d6f0441e00a0"
let f1 = (): Promise.t<unit> => {
  invoke(~cmd="plugin:simple|f1", ~payload={"idlHash": idlHash})
}
let f2 = (a: int): Promise.t<unit> => {
  invoke(~cmd="plugin:simple|f2", ~payload={"idlHash": idlHash, "a": a})
}
let f3 = (a: int, b: int): Promise.t<unit> => {
  invoke(~cmd="plugin:simple|f3", ~payload={"idlHash": idlHash, "a": a, "b": b})
}
let f4 = (): Promise.t<int> => {
  invoke(~cmd="plugin:simple|f4", ~payload={"idlHash": idlHash})
}
let f5 = (): Promise.t<(int, int)> => {
  invoke(~cmd="plugin:simple|f5", ~payload={"idlHash": idlHash})
}
let f6 = (a: int, b: int, c: int): Promise.t<(int, int, int)> => {
  invoke(~cmd="plugin:simple|f6", ~payload={"idlHash": idlHash, "a": a, "b": b, "c": c})
}
