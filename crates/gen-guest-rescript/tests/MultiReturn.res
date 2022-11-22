@scope(("window", "__TAURI__", "tauri"))
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "invoke"
let mra = (): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|mra")->ignore
}
let mrb = (): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|mrb")->ignore
}
let mrc = (): Promise.t<int> => {
  invoke(~cmd="plugin:imports|mrc")->ignore
}
let mrd = (): Promise.t<int> => {
  invoke(~cmd="plugin:imports|mrd")->ignore
}
let mre = (): Promise.t<(int, float)> => {
  invoke(~cmd="plugin:imports|mre")->ignore
}
