@scope(("window", "__TAURI__", "tauri"))
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "invoke"
let a1 = (x: int): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|a1", ~payload={"x": x})->ignore
}
let a2 = (x: int): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|a2", ~payload={"x": x})->ignore
}
let a3 = (x: int): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|a3", ~payload={"x": x})->ignore
}
let a4 = (x: int): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|a4", ~payload={"x": x})->ignore
}
let a5 = (x: int): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|a5", ~payload={"x": x})->ignore
}
let a6 = (x: int): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|a6", ~payload={"x": x})->ignore
}
let a7 = (x: int64): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|a7", ~payload={"x": x})->ignore
}
let a8 = (x: int64): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|a8", ~payload={"x": x})->ignore
}
let a9 = (p1: int, p2: int, p3: int, p4: int, p5: int, p6: int, p7: int64, p8: int64): Promise.t<
  unit,
> => {
  invoke(
    ~cmd="plugin:imports|a9",
    ~payload={"p1": p1, "p2": p2, "p3": p3, "p4": p4, "p5": p5, "p6": p6, "p7": p7, "p8": p8},
  )->ignore
}
let r1 = (): Promise.t<int> => {
  invoke(~cmd="plugin:imports|r1")->ignore
}
let r2 = (): Promise.t<int> => {
  invoke(~cmd="plugin:imports|r2")->ignore
}
let r3 = (): Promise.t<int> => {
  invoke(~cmd="plugin:imports|r3")->ignore
}
let r4 = (): Promise.t<int> => {
  invoke(~cmd="plugin:imports|r4")->ignore
}
let r5 = (): Promise.t<int> => {
  invoke(~cmd="plugin:imports|r5")->ignore
}
let r6 = (): Promise.t<int> => {
  invoke(~cmd="plugin:imports|r6")->ignore
}
let r7 = (): Promise.t<int64> => {
  invoke(~cmd="plugin:imports|r7")->ignore
}
let r8 = (): Promise.t<int64> => {
  invoke(~cmd="plugin:imports|r8")->ignore
}
let pairRet = (): Promise.t<(int64, int)> => {
  invoke(~cmd="plugin:imports|pair_ret")->ignore
}
