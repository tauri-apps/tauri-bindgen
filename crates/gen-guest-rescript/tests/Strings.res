@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
let a = (x: string): Promise.t<unit> => {
  invoke(~cmd="plugin:16c3ebd2|a", ~payload={"x": x})
}
let b = (): Promise.t<string> => {
  invoke(~cmd="plugin:16c3ebd2|b")
}
let c = (a: string, b: string): Promise.t<string> => {
  invoke(~cmd="plugin:16c3ebd2|c", ~payload={"a": a, "b": b})
}
