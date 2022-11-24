@scope("window")
external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
let idlHash = "cccf67b47414af61"
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
  invoke(~cmd="plugin:unions|add_one_integer", ~payload={"idlHash": idlHash, "num": num})
}
let addOneFloat = (num: allFloats): Promise.t<allFloats> => {
  invoke(~cmd="plugin:unions|add_one_float", ~payload={"idlHash": idlHash, "num": num})
}
let replaceFirstChar = (text: allText, letter: char): Promise.t<allText> => {
  invoke(
    ~cmd="plugin:unions|replace_first_char",
    ~payload={"idlHash": idlHash, "text": text, "letter": letter},
  )
}
let identifyInteger = (num: allIntegers): Promise.t<int> => {
  invoke(~cmd="plugin:unions|identify_integer", ~payload={"idlHash": idlHash, "num": num})
}
let identifyFloat = (num: allFloats): Promise.t<int> => {
  invoke(~cmd="plugin:unions|identify_float", ~payload={"idlHash": idlHash, "num": num})
}
let identifyText = (text: allText): Promise.t<int> => {
  invoke(~cmd="plugin:unions|identify_text", ~payload={"idlHash": idlHash, "text": text})
}
let addOneDuplicated = (num: duplicatedS32): Promise.t<duplicatedS32> => {
  invoke(~cmd="plugin:unions|add_one_duplicated", ~payload={"idlHash": idlHash, "num": num})
}
let identifyDuplicated = (num: duplicatedS32): Promise.t<int> => {
  invoke(~cmd="plugin:unions|identify_duplicated", ~payload={"idlHash": idlHash, "num": num})
}
let addOneDistinguishableNum = (num: distinguishableNum): Promise.t<distinguishableNum> => {
  invoke(
    ~cmd="plugin:unions|add_one_distinguishable_num",
    ~payload={"idlHash": idlHash, "num": num},
  )
}
let identifyDistinguishableNum = (num: distinguishableNum): Promise.t<int> => {
  invoke(
    ~cmd="plugin:unions|identify_distinguishable_num",
    ~payload={"idlHash": idlHash, "num": num},
  )
}
