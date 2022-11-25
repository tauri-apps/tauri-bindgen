@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
let mra = (): Promise.t<unit> => {
  invoke(~cmd="plugin:d238f570|mra")
}
let mrb = (): Promise.t<unit> => {
  invoke(~cmd="plugin:d238f570|mrb")
}
let mrc = (): Promise.t<int> => {
  invoke(~cmd="plugin:d238f570|mrc")
}
let mrd = (): Promise.t<int> => {
  invoke(~cmd="plugin:d238f570|mrd")
}
let mre = (): Promise.t<(int, float)> => {
  invoke(~cmd="plugin:d238f570|mre")
}
