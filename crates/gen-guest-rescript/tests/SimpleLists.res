@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
let simpleList1 = (l: TypedArray.uint32Array): Promise.t<unit> => {
  invoke(~cmd="plugin:d40a3203ef48115d|simple_list1", ~payload={"l": l})
}
let simpleList2 = (): Promise.t<TypedArray.uint32Array> => {
  invoke(~cmd="plugin:d40a3203ef48115d|simple_list2")
}
let simpleList4 = (l: array<TypedArray.uint32Array>): Promise.t<array<TypedArray.uint32Array>> => {
  invoke(~cmd="plugin:d40a3203ef48115d|simple_list4", ~payload={"l": l})
}
