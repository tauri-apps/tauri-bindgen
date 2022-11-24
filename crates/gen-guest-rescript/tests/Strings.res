@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
let idlHash = "16c3ebd2deefea81"
let a = (x: string): Promise.t<unit> => {
  invoke(~cmd="plugin:strings|a", ~payload={"idlHash": idlHash, "x": x})
}
let b = (): Promise.t<string> => {
  invoke(~cmd="plugin:strings|b", ~payload={"idlHash": idlHash})
}
let c = (a: string, b: string): Promise.t<string> => {
  invoke(~cmd="plugin:strings|c", ~payload={"idlHash": idlHash, "a": a, "b": b})
}
