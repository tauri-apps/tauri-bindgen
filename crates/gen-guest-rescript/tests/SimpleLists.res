@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
let idlHash = "d40a3203ef48115d"
let simpleList1 = (l: TypedArray.uint32Array): Promise.t<unit> => {
  invoke(~cmd="plugin:simple_lists|simple_list1", ~payload={"idlHash": idlHash, "l": l})
}
let simpleList2 = (): Promise.t<TypedArray.uint32Array> => {
  invoke(~cmd="plugin:simple_lists|simple_list2", ~payload={"idlHash": idlHash})
}
let simpleList4 = (l: array<TypedArray.uint32Array>): Promise.t<array<TypedArray.uint32Array>> => {
  invoke(~cmd="plugin:simple_lists|simple_list4", ~payload={"idlHash": idlHash, "l": l})
}
