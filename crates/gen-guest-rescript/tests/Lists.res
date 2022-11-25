@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
type someRecord = {
  x: string,
  y: otherRecord,
  z: array<otherRecord>,
  c1: int,
  c2: int64,
  c3: int,
  c4: int64,
}
type otherRecord = {
  a1: int,
  a2: int64,
  a3: int,
  a4: int64,
  b: string,
  c: TypedArray.uint8Array,
}
type someVariant =
  | A(string)
  | B
  | C(int)
  | D(array<otherVariant>)

type otherVariant =
  | A
  | B(int)
  | C(string)

type loadStoreAllSizes = array<(
  string,
  int,
  int,
  int,
  int,
  int,
  int,
  int64,
  int64,
  float,
  float,
  char,
)>
let listU8Param = (x: TypedArray.uint8Array): Promise.t<unit> => {
  invoke(~cmd="plugin:a744d1c6|list_u8_param", ~payload={"x": x})
}
let listU16Param = (x: TypedArray.uint16Array): Promise.t<unit> => {
  invoke(~cmd="plugin:a744d1c6|list_u16_param", ~payload={"x": x})
}
let listU32Param = (x: TypedArray.uint32Array): Promise.t<unit> => {
  invoke(~cmd="plugin:a744d1c6|list_u32_param", ~payload={"x": x})
}
let listU64Param = (x: array<int64>): Promise.t<unit> => {
  invoke(~cmd="plugin:a744d1c6|list_u64_param", ~payload={"x": x})
}
let listS8Param = (x: TypedArray.int8Array): Promise.t<unit> => {
  invoke(~cmd="plugin:a744d1c6|list_s8_param", ~payload={"x": x})
}
let listS16Param = (x: TypedArray.int16Array): Promise.t<unit> => {
  invoke(~cmd="plugin:a744d1c6|list_s16_param", ~payload={"x": x})
}
let listS32Param = (x: TypedArray.int32Array): Promise.t<unit> => {
  invoke(~cmd="plugin:a744d1c6|list_s32_param", ~payload={"x": x})
}
let listS64Param = (x: array<int64>): Promise.t<unit> => {
  invoke(~cmd="plugin:a744d1c6|list_s64_param", ~payload={"x": x})
}
let listFloat32Param = (x: TypedArray.float32Array): Promise.t<unit> => {
  invoke(~cmd="plugin:a744d1c6|list_float32_param", ~payload={"x": x})
}
let listFloat64Param = (x: TypedArray.float64Array): Promise.t<unit> => {
  invoke(~cmd="plugin:a744d1c6|list_float64_param", ~payload={"x": x})
}
let listU8Ret = (): Promise.t<TypedArray.uint8Array> => {
  invoke(~cmd="plugin:a744d1c6|list_u8_ret")
}
let listU16Ret = (): Promise.t<TypedArray.uint16Array> => {
  invoke(~cmd="plugin:a744d1c6|list_u16_ret")
}
let listU32Ret = (): Promise.t<TypedArray.uint32Array> => {
  invoke(~cmd="plugin:a744d1c6|list_u32_ret")
}
let listU64Ret = (): Promise.t<array<int64>> => {
  invoke(~cmd="plugin:a744d1c6|list_u64_ret")
}
let listS8Ret = (): Promise.t<TypedArray.int8Array> => {
  invoke(~cmd="plugin:a744d1c6|list_s8_ret")
}
let listS16Ret = (): Promise.t<TypedArray.int16Array> => {
  invoke(~cmd="plugin:a744d1c6|list_s16_ret")
}
let listS32Ret = (): Promise.t<TypedArray.int32Array> => {
  invoke(~cmd="plugin:a744d1c6|list_s32_ret")
}
let listS64Ret = (): Promise.t<array<int64>> => {
  invoke(~cmd="plugin:a744d1c6|list_s64_ret")
}
let listFloat32Ret = (): Promise.t<TypedArray.float32Array> => {
  invoke(~cmd="plugin:a744d1c6|list_float32_ret")
}
let listFloat64Ret = (): Promise.t<TypedArray.float64Array> => {
  invoke(~cmd="plugin:a744d1c6|list_float64_ret")
}
let tupleList = (x: array<(int, int)>): Promise.t<array<(int64, int)>> => {
  invoke(~cmd="plugin:a744d1c6|tuple_list", ~payload={"x": x})
}
let stringListArg = (a: array<string>): Promise.t<unit> => {
  invoke(~cmd="plugin:a744d1c6|string_list_arg", ~payload={"a": a})
}
let stringListRet = (): Promise.t<array<string>> => {
  invoke(~cmd="plugin:a744d1c6|string_list_ret")
}
let tupleStringList = (x: array<(int, string)>): Promise.t<array<(string, int)>> => {
  invoke(~cmd="plugin:a744d1c6|tuple_string_list", ~payload={"x": x})
}
let stringList = (x: array<string>): Promise.t<array<string>> => {
  invoke(~cmd="plugin:a744d1c6|string_list", ~payload={"x": x})
}
let recordList = (x: array<someRecord>): Promise.t<array<otherRecord>> => {
  invoke(~cmd="plugin:a744d1c6|record_list", ~payload={"x": x})
}
let recordListReverse = (x: array<otherRecord>): Promise.t<array<someRecord>> => {
  invoke(~cmd="plugin:a744d1c6|record_list_reverse", ~payload={"x": x})
}
let variantList = (x: array<someVariant>): Promise.t<array<otherVariant>> => {
  invoke(~cmd="plugin:a744d1c6|variant_list", ~payload={"x": x})
}
let loadStoreEverything = (a: loadStoreAllSizes): Promise.t<loadStoreAllSizes> => {
  invoke(~cmd="plugin:a744d1c6|load_store_everything", ~payload={"a": a})
}
