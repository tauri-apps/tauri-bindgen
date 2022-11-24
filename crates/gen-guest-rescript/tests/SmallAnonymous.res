@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
let idlHash = "bee731db80799df9"
type error =
  | Success
  | Failure

let optionTest = (): Promise.t<option<string>> => {
  invoke(~cmd="plugin:anon|option_test", ~payload={"idlHash": idlHash})
}
