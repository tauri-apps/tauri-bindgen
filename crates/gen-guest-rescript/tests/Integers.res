@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
let a1 = (x: int): Promise.t<unit> => {
  invoke(~cmd="plugin:279b557e|a1", ~payload={"x": x})
}
let a2 = (x: int): Promise.t<unit> => {
  invoke(~cmd="plugin:279b557e|a2", ~payload={"x": x})
}
let a3 = (x: int): Promise.t<unit> => {
  invoke(~cmd="plugin:279b557e|a3", ~payload={"x": x})
}
let a4 = (x: int): Promise.t<unit> => {
  invoke(~cmd="plugin:279b557e|a4", ~payload={"x": x})
}
let a5 = (x: int): Promise.t<unit> => {
  invoke(~cmd="plugin:279b557e|a5", ~payload={"x": x})
}
let a6 = (x: int): Promise.t<unit> => {
  invoke(~cmd="plugin:279b557e|a6", ~payload={"x": x})
}
let a7 = (x: int64): Promise.t<unit> => {
  invoke(~cmd="plugin:279b557e|a7", ~payload={"x": x})
}
let a8 = (x: int64): Promise.t<unit> => {
  invoke(~cmd="plugin:279b557e|a8", ~payload={"x": x})
}
let a9 = (p1: int, p2: int, p3: int, p4: int, p5: int, p6: int, p7: int64, p8: int64): Promise.t<
  unit,
> => {
  invoke(
    ~cmd="plugin:279b557e|a9",
    ~payload={"p1": p1, "p2": p2, "p3": p3, "p4": p4, "p5": p5, "p6": p6, "p7": p7, "p8": p8},
  )
}
let r1 = (): Promise.t<int> => {
  invoke(~cmd="plugin:279b557e|r1")
}
let r2 = (): Promise.t<int> => {
  invoke(~cmd="plugin:279b557e|r2")
}
let r3 = (): Promise.t<int> => {
  invoke(~cmd="plugin:279b557e|r3")
}
let r4 = (): Promise.t<int> => {
  invoke(~cmd="plugin:279b557e|r4")
}
let r5 = (): Promise.t<int> => {
  invoke(~cmd="plugin:279b557e|r5")
}
let r6 = (): Promise.t<int> => {
  invoke(~cmd="plugin:279b557e|r6")
}
let r7 = (): Promise.t<int64> => {
  invoke(~cmd="plugin:279b557e|r7")
}
let r8 = (): Promise.t<int64> => {
  invoke(~cmd="plugin:279b557e|r8")
}
let pairRet = (): Promise.t<(int64, int)> => {
  invoke(~cmd="plugin:279b557e|pair_ret")
}
