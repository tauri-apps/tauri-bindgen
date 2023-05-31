# lists



## Type definitions

## Struct other_record



### Fields

#### a1: `u32`

#### a2: `u64`

#### a3: `s32`

#### a4: `s64`

#### b: `string`

#### c: `list<u8>`


## Struct some_record



### Fields

#### x: `string`

#### y: `[other_record](#other_record)`

#### z: `list<[other_record](#other_record)>`

#### c1: `u32`

#### c2: `u64`

#### c3: `s32`

#### c4: `s64`


## Variant other_variant



### Cases

#### a

#### b: `u32`

#### c: `string`


## Variant some_variant



### Cases

#### a: `string`

#### b

#### c: `u32`

#### d: `list<[other_variant](#other_variant)>`


## Alias load_store_all_sizes

`list<tuple<string, u8, s8, u16, s16, u32, s32, u64, s64, float32, float64, char>>`



## Functions

### Function list_u8_param

`func list_u8_param (x: list<u8>)`


### Function list_u16_param

`func list_u16_param (x: list<u16>)`


### Function list_u32_param

`func list_u32_param (x: list<u32>)`


### Function list_u64_param

`func list_u64_param (x: list<u64>)`


### Function list_u128_param

`func list_u128_param (x: list<u128>)`


### Function list_s8_param

`func list_s8_param (x: list<s8>)`


### Function list_s16_param

`func list_s16_param (x: list<s16>)`


### Function list_s32_param

`func list_s32_param (x: list<s32>)`


### Function list_s64_param

`func list_s64_param (x: list<s64>)`


### Function list_s128_param

`func list_s128_param (x: list<s128>)`


### Function list_float32_param

`func list_float32_param (x: list<float32>)`


### Function list_float64_param

`func list_float64_param (x: list<float64>)`


### Function list_u8_ret

`func list_u8_ret () -> list<u8>`


### Function list_u16_ret

`func list_u16_ret () -> list<u16>`


### Function list_u32_ret

`func list_u32_ret () -> list<u32>`


### Function list_u64_ret

`func list_u64_ret () -> list<u64>`


### Function list_u128_ret

`func list_u128_ret () -> list<u128>`


### Function list_s8_ret

`func list_s8_ret () -> list<s8>`


### Function list_s16_ret

`func list_s16_ret () -> list<s16>`


### Function list_s32_ret

`func list_s32_ret () -> list<s32>`


### Function list_s64_ret

`func list_s64_ret () -> list<s64>`


### Function list_s128_ret

`func list_s128_ret () -> list<s128>`


### Function list_float32_ret

`func list_float32_ret () -> list<float32>`


### Function list_float64_ret

`func list_float64_ret () -> list<float64>`


### Function tuple_list

`func tuple_list (x: list<tuple<u8, s8>>) -> list<tuple<s64, u32>>`


### Function string_list_arg

`func string_list_arg (a: list<string>)`


### Function string_list_ret

`func string_list_ret () -> list<string>`


### Function tuple_string_list

`func tuple_string_list (x: list<tuple<u8, string>>) -> list<tuple<string, u8>>`


### Function string_list

`func string_list (x: list<string>) -> list<string>`


### Function record_list

`func record_list (x: list<[some_record](#some_record)>) -> list<[other_record](#other_record)>`


### Function record_list_reverse

`func record_list_reverse (x: list<[other_record](#other_record)>) -> list<[some_record](#some_record)>`


### Function variant_list

`func variant_list (x: list<[some_variant](#some_variant)>) -> list<[other_variant](#other_variant)>`


### Function load_store_everything

`func load_store_everything (a: [load_store_all_sizes](#load_store_all_sizes)) -> [load_store_all_sizes](#load_store_all_sizes)`

