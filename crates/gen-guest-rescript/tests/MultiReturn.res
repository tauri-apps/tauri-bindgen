@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
let idlHash = "d238f57052cdcb90"
let mra = (): Promise.t<unit> => {
  invoke(~cmd="plugin:multi_return|mra", ~payload={"idlHash": idlHash})
}
let mrb = (): Promise.t<unit> => {
  invoke(~cmd="plugin:multi_return|mrb", ~payload={"idlHash": idlHash})
}
let mrc = (): Promise.t<int> => {
  invoke(~cmd="plugin:multi_return|mrc", ~payload={"idlHash": idlHash})
}
let mrd = (): Promise.t<int> => {
  invoke(~cmd="plugin:multi_return|mrd", ~payload={"idlHash": idlHash})
}
let mre = (): Promise.t<(int, float)> => {
  invoke(~cmd="plugin:multi_return|mre", ~payload={"idlHash": idlHash})
}
