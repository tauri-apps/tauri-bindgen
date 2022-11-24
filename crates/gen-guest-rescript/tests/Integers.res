@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
if Belt.Option.isNone(%external(__TAURI_BINDGEN_VERSION_CHECK__)) {
  invoke(~cmd="plugin:integers|279b557e344c2e05853f5c89d6d511dc")->catch(e => {
    Js.Console.error(
      "The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: You can disable this check by setting `window.__TAURI_BINDGEN_VERSION_CHECK__` to `false`.",
    )
  })
}
let a1 = (x: int): Promise.t<unit> => {
  invoke(~cmd="plugin:integers|a1", ~payload={"x": x})
}
let a2 = (x: int): Promise.t<unit> => {
  invoke(~cmd="plugin:integers|a2", ~payload={"x": x})
}
let a3 = (x: int): Promise.t<unit> => {
  invoke(~cmd="plugin:integers|a3", ~payload={"x": x})
}
let a4 = (x: int): Promise.t<unit> => {
  invoke(~cmd="plugin:integers|a4", ~payload={"x": x})
}
let a5 = (x: int): Promise.t<unit> => {
  invoke(~cmd="plugin:integers|a5", ~payload={"x": x})
}
let a6 = (x: int): Promise.t<unit> => {
  invoke(~cmd="plugin:integers|a6", ~payload={"x": x})
}
let a7 = (x: int64): Promise.t<unit> => {
  invoke(~cmd="plugin:integers|a7", ~payload={"x": x})
}
let a8 = (x: int64): Promise.t<unit> => {
  invoke(~cmd="plugin:integers|a8", ~payload={"x": x})
}
let a9 = (p1: int, p2: int, p3: int, p4: int, p5: int, p6: int, p7: int64, p8: int64): Promise.t<
  unit,
> => {
  invoke(
    ~cmd="plugin:integers|a9",
    ~payload={"p1": p1, "p2": p2, "p3": p3, "p4": p4, "p5": p5, "p6": p6, "p7": p7, "p8": p8},
  )
}
let r1 = (): Promise.t<int> => {
  invoke(~cmd="plugin:integers|r1")
}
let r2 = (): Promise.t<int> => {
  invoke(~cmd="plugin:integers|r2")
}
let r3 = (): Promise.t<int> => {
  invoke(~cmd="plugin:integers|r3")
}
let r4 = (): Promise.t<int> => {
  invoke(~cmd="plugin:integers|r4")
}
let r5 = (): Promise.t<int> => {
  invoke(~cmd="plugin:integers|r5")
}
let r6 = (): Promise.t<int> => {
  invoke(~cmd="plugin:integers|r6")
}
let r7 = (): Promise.t<int64> => {
  invoke(~cmd="plugin:integers|r7")
}
let r8 = (): Promise.t<int64> => {
  invoke(~cmd="plugin:integers|r8")
}
let pairRet = (): Promise.t<(int64, int)> => {
  invoke(~cmd="plugin:integers|pair_ret")
}
