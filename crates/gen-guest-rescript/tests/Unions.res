@scope(("window", "__TAURI__", "tauri"))
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "invoke"
/**
* A union of all of the integral types
*/
type allIntegers =
  | Bool(bool)
  | U8(int)
  | U16(int)
  | U32(int)
  | U64(int64)
  | I8(int)
  | I16(int)
  | I32(int)
  | I64(int64)

type allFloats =
  | F32(float)
  | F64(float)

type allText =
  | Char(char)
  | String(string)

type duplicatedS32 =
  | I32(int)
  | I32(int)
  | I32(int)

/**
* A type containing numeric types that are distinct in most languages
*/
type distinguishableNum =
  | F64(float)
  | I64(int64)

let addOneInteger = (num: allIntegers): Promise.t<allIntegers> => {
  invoke(~cmd="plugin:import_unions|add_one_integer", ~payload={"num": num})->ignore
}
let addOneFloat = (num: allFloats): Promise.t<allFloats> => {
  invoke(~cmd="plugin:import_unions|add_one_float", ~payload={"num": num})->ignore
}
let replaceFirstChar = (text: allText, letter: char): Promise.t<allText> => {
  invoke(
    ~cmd="plugin:import_unions|replace_first_char",
    ~payload={"text": text, "letter": letter},
  )->ignore
}
let identifyInteger = (num: allIntegers): Promise.t<int> => {
  invoke(~cmd="plugin:import_unions|identify_integer", ~payload={"num": num})->ignore
}
let identifyFloat = (num: allFloats): Promise.t<int> => {
  invoke(~cmd="plugin:import_unions|identify_float", ~payload={"num": num})->ignore
}
let identifyText = (text: allText): Promise.t<int> => {
  invoke(~cmd="plugin:import_unions|identify_text", ~payload={"text": text})->ignore
}
let addOneDuplicated = (num: duplicatedS32): Promise.t<duplicatedS32> => {
  invoke(~cmd="plugin:import_unions|add_one_duplicated", ~payload={"num": num})->ignore
}
let identifyDuplicated = (num: duplicatedS32): Promise.t<int> => {
  invoke(~cmd="plugin:import_unions|identify_duplicated", ~payload={"num": num})->ignore
}
let addOneDistinguishableNum = (num: distinguishableNum): Promise.t<distinguishableNum> => {
  invoke(~cmd="plugin:import_unions|add_one_distinguishable_num", ~payload={"num": num})->ignore
}
let identifyDistinguishableNum = (num: distinguishableNum): Promise.t<int> => {
  invoke(~cmd="plugin:import_unions|identify_distinguishable_num", ~payload={"num": num})->ignore
}
