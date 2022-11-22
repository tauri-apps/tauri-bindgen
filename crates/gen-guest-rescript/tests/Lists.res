@scope(("window", "__TAURI__", "tauri"))
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "invoke"
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
  invoke(~cmd="plugin:import_lists|list_u8_param", ~payload={"x": x})->ignore
}
let listU16Param = (x: TypedArray.uint16Array): Promise.t<unit> => {
  invoke(~cmd="plugin:import_lists|list_u16_param", ~payload={"x": x})->ignore
}
let listU32Param = (x: TypedArray.uint32Array): Promise.t<unit> => {
  invoke(~cmd="plugin:import_lists|list_u32_param", ~payload={"x": x})->ignore
}
let listU64Param = (x: array<int64>): Promise.t<unit> => {
  invoke(~cmd="plugin:import_lists|list_u64_param", ~payload={"x": x})->ignore
}
let listS8Param = (x: TypedArray.int8Array): Promise.t<unit> => {
  invoke(~cmd="plugin:import_lists|list_s8_param", ~payload={"x": x})->ignore
}
let listS16Param = (x: TypedArray.int16Array): Promise.t<unit> => {
  invoke(~cmd="plugin:import_lists|list_s16_param", ~payload={"x": x})->ignore
}
let listS32Param = (x: TypedArray.int32Array): Promise.t<unit> => {
  invoke(~cmd="plugin:import_lists|list_s32_param", ~payload={"x": x})->ignore
}
let listS64Param = (x: array<int64>): Promise.t<unit> => {
  invoke(~cmd="plugin:import_lists|list_s64_param", ~payload={"x": x})->ignore
}
let listFloat32Param = (x: TypedArray.float32Array): Promise.t<unit> => {
  invoke(~cmd="plugin:import_lists|list_float32_param", ~payload={"x": x})->ignore
}
let listFloat64Param = (x: TypedArray.float64Array): Promise.t<unit> => {
  invoke(~cmd="plugin:import_lists|list_float64_param", ~payload={"x": x})->ignore
}
let listU8Ret = (): Promise.t<TypedArray.uint8Array> => {
  invoke(~cmd="plugin:import_lists|list_u8_ret")->ignore
}
let listU16Ret = (): Promise.t<TypedArray.uint16Array> => {
  invoke(~cmd="plugin:import_lists|list_u16_ret")->ignore
}
let listU32Ret = (): Promise.t<TypedArray.uint32Array> => {
  invoke(~cmd="plugin:import_lists|list_u32_ret")->ignore
}
let listU64Ret = (): Promise.t<array<int64>> => {
  invoke(~cmd="plugin:import_lists|list_u64_ret")->ignore
}
let listS8Ret = (): Promise.t<TypedArray.int8Array> => {
  invoke(~cmd="plugin:import_lists|list_s8_ret")->ignore
}
let listS16Ret = (): Promise.t<TypedArray.int16Array> => {
  invoke(~cmd="plugin:import_lists|list_s16_ret")->ignore
}
let listS32Ret = (): Promise.t<TypedArray.int32Array> => {
  invoke(~cmd="plugin:import_lists|list_s32_ret")->ignore
}
let listS64Ret = (): Promise.t<array<int64>> => {
  invoke(~cmd="plugin:import_lists|list_s64_ret")->ignore
}
let listFloat32Ret = (): Promise.t<TypedArray.float32Array> => {
  invoke(~cmd="plugin:import_lists|list_float32_ret")->ignore
}
let listFloat64Ret = (): Promise.t<TypedArray.float64Array> => {
  invoke(~cmd="plugin:import_lists|list_float64_ret")->ignore
}
let tupleList = (x: array<(int, int)>): Promise.t<array<(int64, int)>> => {
  invoke(~cmd="plugin:import_lists|tuple_list", ~payload={"x": x})->ignore
}
let stringListArg = (a: array<string>): Promise.t<unit> => {
  invoke(~cmd="plugin:import_lists|string_list_arg", ~payload={"a": a})->ignore
}
let stringListRet = (): Promise.t<array<string>> => {
  invoke(~cmd="plugin:import_lists|string_list_ret")->ignore
}
let tupleStringList = (x: array<(int, string)>): Promise.t<array<(string, int)>> => {
  invoke(~cmd="plugin:import_lists|tuple_string_list", ~payload={"x": x})->ignore
}
let stringList = (x: array<string>): Promise.t<array<string>> => {
  invoke(~cmd="plugin:import_lists|string_list", ~payload={"x": x})->ignore
}
let recordList = (x: array<someRecord>): Promise.t<array<otherRecord>> => {
  invoke(~cmd="plugin:import_lists|record_list", ~payload={"x": x})->ignore
}
let recordListReverse = (x: array<otherRecord>): Promise.t<array<someRecord>> => {
  invoke(~cmd="plugin:import_lists|record_list_reverse", ~payload={"x": x})->ignore
}
let variantList = (x: array<someVariant>): Promise.t<array<otherVariant>> => {
  invoke(~cmd="plugin:import_lists|variant_list", ~payload={"x": x})->ignore
}
let loadStoreEverything = (a: loadStoreAllSizes): Promise.t<loadStoreAllSizes> => {
  invoke(~cmd="plugin:import_lists|load_store_everything", ~payload={"a": a})->ignore
}
