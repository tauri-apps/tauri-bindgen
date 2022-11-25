@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
type error =
  | Success
  | Failure

let optionTest = (): Promise.t<option<string>> => {
  invoke(~cmd="plugin:bee731db80799df9|option_test")
}
