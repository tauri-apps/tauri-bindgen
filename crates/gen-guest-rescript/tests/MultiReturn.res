@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
let mra = (): Promise.t<unit> => {
  invoke(~cmd="plugin:d238f57052cdcb90|mra")
}
let mrb = (): Promise.t<unit> => {
  invoke(~cmd="plugin:d238f57052cdcb90|mrb")
}
let mrc = (): Promise.t<int> => {
  invoke(~cmd="plugin:d238f57052cdcb90|mrc")
}
let mrd = (): Promise.t<int> => {
  invoke(~cmd="plugin:d238f57052cdcb90|mrd")
}
let mre = (): Promise.t<(int, float)> => {
  invoke(~cmd="plugin:d238f57052cdcb90|mre")
}
