@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
let f1 = (): Promise.t<unit> => {
  invoke(~cmd="plugin:ebb2d6f0441e00a0|f1")
}
let f2 = (a: int): Promise.t<unit> => {
  invoke(~cmd="plugin:ebb2d6f0441e00a0|f2", ~payload={"a": a})
}
let f3 = (a: int, b: int): Promise.t<unit> => {
  invoke(~cmd="plugin:ebb2d6f0441e00a0|f3", ~payload={"a": a, "b": b})
}
let f4 = (): Promise.t<int> => {
  invoke(~cmd="plugin:ebb2d6f0441e00a0|f4")
}
let f5 = (): Promise.t<(int, int)> => {
  invoke(~cmd="plugin:ebb2d6f0441e00a0|f5")
}
let f6 = (a: int, b: int, c: int): Promise.t<(int, int, int)> => {
  invoke(~cmd="plugin:ebb2d6f0441e00a0|f6", ~payload={"a": a, "b": b, "c": c})
}
