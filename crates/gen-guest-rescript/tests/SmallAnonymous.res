@scope(("window", "__TAURI__", "tauri"))
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "invoke"
type error =
  | Success
  | Failure

let optionTest = (): Promise.t<option<string>> => {
  invoke(~cmd="plugin:imports|option_test")->ignore
}
