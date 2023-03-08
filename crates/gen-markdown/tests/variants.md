# variants



## Type definitions

## Enum e1



### Cases

#### a


## Enum e1



### Cases

#### a


## Union u1



### Cases

#### `u32`

#### `float32`


## Union u1



### Cases

#### `u32`

#### `float32`


## Union u1



### Cases

#### `u32`

#### `float32`


## Enum e1



### Cases

#### a


## Struct empty



### Fields


## Variant v1



### Cases

#### a

#### b: `[u1](#u1)`

#### c: `[e1](#e1)`

#### d: `string`

#### e: `[empty](#empty)`

#### f

#### g: `u32`


## Union u1



### Cases

#### `u32`

#### `float32`


## Enum e1



### Cases

#### a


## Struct empty



### Fields


## Variant v1



### Cases

#### a

#### b: `[u1](#u1)`

#### c: `[e1](#e1)`

#### d: `string`

#### e: `[empty](#empty)`

#### f

#### g: `u32`


## Enum e1



### Cases

#### a


## Union u1



### Cases

#### `u32`

#### `float32`


## Enum e1



### Cases

#### a


## Union u1



### Cases

#### `u32`

#### `float32`


## Variant casts1



### Cases

#### a: `s32`

#### b: `float32`


## Variant casts2



### Cases

#### a: `float64`

#### b: `float32`


## Variant casts3



### Cases

#### a: `float64`

#### b: `u64`


## Variant casts4



### Cases

#### a: `u32`

#### b: `s64`


## Variant casts5



### Cases

#### a: `float32`

#### b: `s64`


## Variant casts6



### Cases

#### a: `tuple<float32, u32>`

#### b: `tuple<u32, u32>`


## Variant casts1



### Cases

#### a: `s32`

#### b: `float32`


## Variant casts2



### Cases

#### a: `float64`

#### b: `float32`


## Variant casts3



### Cases

#### a: `float64`

#### b: `u64`


## Variant casts4



### Cases

#### a: `u32`

#### b: `s64`


## Variant casts5



### Cases

#### a: `float32`

#### b: `s64`


## Variant casts6



### Cases

#### a: `tuple<float32, u32>`

#### b: `tuple<u32, u32>`


## Enum e1



### Cases

#### a


## Enum e1



### Cases

#### a


## Union u1



### Cases

#### `u32`

#### `float32`


## Enum e1



### Cases

#### a


## Struct empty



### Fields


## Variant v1



### Cases

#### a

#### b: `[u1](#u1)`

#### c: `[e1](#e1)`

#### d: `string`

#### e: `[empty](#empty)`

#### f

#### g: `u32`


## Enum e1



### Cases

#### a


## Enum e1



### Cases

#### a


## Union u1



### Cases

#### `u32`

#### `float32`


## Enum e1



### Cases

#### a


## Struct empty



### Fields


## Variant v1



### Cases

#### a

#### b: `[u1](#u1)`

#### c: `[e1](#e1)`

#### d: `string`

#### e: `[empty](#empty)`

#### f

#### g: `u32`


## Enum my_errno



### Cases

#### bad1

#### bad2


## Enum my_errno



### Cases

#### bad1

#### bad2


## Enum my_errno



### Cases

#### bad1

#### bad2


## Enum my_errno



### Cases

#### bad1

#### bad2


## Enum my_errno



### Cases

#### bad1

#### bad2


## Enum my_errno



### Cases

#### bad1

#### bad2


## Union u1



### Cases

#### `u32`

#### `float32`


## Enum e1



### Cases

#### a


## Struct empty



### Fields


## Variant v1



### Cases

#### a

#### b: `[u1](#u1)`

#### c: `[e1](#e1)`

#### d: `string`

#### e: `[empty](#empty)`

#### f

#### g: `u32`


## Struct is_clone



### Fields

#### v1: `[v1](#v1)`


## Union u1



### Cases

#### `u32`

#### `float32`


## Enum e1



### Cases

#### a


## Struct empty



### Fields


## Variant v1



### Cases

#### a

#### b: `[u1](#u1)`

#### c: `[e1](#e1)`

#### d: `string`

#### e: `[empty](#empty)`

#### f

#### g: `u32`


## Struct is_clone



### Fields

#### v1: `[v1](#v1)`


## Enum my_errno



### Cases

#### bad1

#### bad2



## Functions

### Function e1_arg

`func e1_arg (x: [e1](#e1))`


### Function e1_result

`func e1_result () -> [e1](#e1)`


### Function u1_arg

`func u1_arg (x: [u1](#u1))`


### Function u1_result

`func u1_result () -> [u1](#u1)`


### Function v1_arg

`func v1_arg (x: [v1](#v1))`


### Function v1_result

`func v1_result () -> [v1](#v1)`


### Function bool_arg

`func bool_arg (x: bool)`


### Function bool_result

`func bool_result () -> bool`


### Function option_arg

`func option_arg (a: option<bool>, b: option<tuple<>>, c: option<u32>, d: option<[e1](#e1)>, e: option<float32>, f: option<[u1](#u1)>, g: option<option<bool>>)`


### Function option_result

`func option_result () -> tuple<option<bool>, option<tuple<>>, option<u32>, option<[e1](#e1)>, option<float32>, option<[u1](#u1)>, option<option<bool>>>`


### Function casts

`func casts (a: [casts1](#casts1), b: [casts2](#casts2), c: [casts3](#casts3), d: [casts4](#casts4), e: [casts5](#casts5), f: [casts6](#casts6)) -> tuple<[casts1](#casts1), [casts2](#casts2), [casts3](#casts3), [casts4](#casts4), [casts5](#casts5), [casts6](#casts6)>`


### Function result_arg

`func result_arg (a: result<_, _>, b: result<_, [e1](#e1)>, c: result<[e1](#e1), _>, d: result<tuple<>, tuple<>>, e: result<u32, [v1](#v1)>, f: result<string, list<u8>>)`


### Function result_result

`func result_result () -> tuple<result<_, _>, result<_, [e1](#e1)>, result<[e1](#e1), _>, result<tuple<>, tuple<>>, result<u32, [v1](#v1)>, result<string, list<u8>>>`


### Function return_result_sugar

`func return_result_sugar () -> result<s32, [my_errno](#my_errno)>`


### Function return_result_sugar2

`func return_result_sugar2 () -> result<_, [my_errno](#my_errno)>`


### Function return_result_sugar3

`func return_result_sugar3 () -> result<[my_errno](#my_errno), [my_errno](#my_errno)>`


### Function return_result_sugar4

`func return_result_sugar4 () -> result<tuple<s32, u32>, [my_errno](#my_errno)>`


### Function return_option_sugar

`func return_option_sugar () -> option<s32>`


### Function return_option_sugar2

`func return_option_sugar2 () -> option<[my_errno](#my_errno)>`


### Function result_simple

`func result_simple () -> result<u32, s32>`


### Function is_clone_arg

`func is_clone_arg (a: [is_clone](#is_clone))`


### Function is_clone_return

`func is_clone_return () -> [is_clone](#is_clone)`


### Function return_named_option

`func return_named_option () -> (a: option<u8>)`


### Function return_named_result

`func return_named_result () -> (a: result<u8, [my_errno](#my_errno)>)`

