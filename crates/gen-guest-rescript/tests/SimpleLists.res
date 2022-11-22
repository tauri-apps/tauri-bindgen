@scope(("window", "__TAURI__", "tauri"))
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "invoke"
let simpleList1 = (l: TypedArray.uint32Array): Promise.t<unit> => {
  invoke(~cmd="plugin:imports|simple_list1", ~payload={"l": l})->ignore
}
let simpleList2 = (): Promise.t<TypedArray.uint32Array> => {
  invoke(~cmd="plugin:imports|simple_list2")->ignore
}
let simpleList4 = (l: array<TypedArray.uint32Array>): Promise.t<array<TypedArray.uint32Array>> => {
  invoke(~cmd="plugin:imports|simple_list4", ~payload={"l": l})->ignore
}
