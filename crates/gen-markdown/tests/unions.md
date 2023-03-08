# unions



## Type definitions

## Union all_integers

A union of all of the integral types

### Cases

#### `bool`
Bool is equivalent to a 1 bit integer
and is treated that way in some languages
#### `u8`

#### `u16`

#### `u32`

#### `u64`

#### `s8`

#### `s16`

#### `s32`

#### `s64`


## Union all_integers

A union of all of the integral types

### Cases

#### `bool`
Bool is equivalent to a 1 bit integer
and is treated that way in some languages
#### `u8`

#### `u16`

#### `u32`

#### `u64`

#### `s8`

#### `s16`

#### `s32`

#### `s64`


## Union all_floats



### Cases

#### `float32`

#### `float64`


## Union all_floats



### Cases

#### `float32`

#### `float64`


## Union all_text



### Cases

#### `char`

#### `string`


## Union all_text



### Cases

#### `char`

#### `string`


## Union all_integers

A union of all of the integral types

### Cases

#### `bool`
Bool is equivalent to a 1 bit integer
and is treated that way in some languages
#### `u8`

#### `u16`

#### `u32`

#### `u64`

#### `s8`

#### `s16`

#### `s32`

#### `s64`


## Union all_floats



### Cases

#### `float32`

#### `float64`


## Union all_text



### Cases

#### `char`

#### `string`


## Union duplicated_s32



### Cases

#### `s32`
The first s32
#### `s32`
The second s32
#### `s32`
The third s32

## Union duplicated_s32



### Cases

#### `s32`
The first s32
#### `s32`
The second s32
#### `s32`
The third s32

## Union duplicated_s32



### Cases

#### `s32`
The first s32
#### `s32`
The second s32
#### `s32`
The third s32

## Union distinguishable_num

A type containing numeric types that are distinct in most languages

### Cases

#### `float64`
A Floating Point Number
#### `s64`
A Signed Integer

## Union distinguishable_num

A type containing numeric types that are distinct in most languages

### Cases

#### `float64`
A Floating Point Number
#### `s64`
A Signed Integer

## Union distinguishable_num

A type containing numeric types that are distinct in most languages

### Cases

#### `float64`
A Floating Point Number
#### `s64`
A Signed Integer


## Functions

### Function add_one_integer

`func add_one_integer (num: [all_integers](#all_integers)) -> [all_integers](#all_integers)`


### Function add_one_float

`func add_one_float (num: [all_floats](#all_floats)) -> [all_floats](#all_floats)`


### Function replace_first_char

`func replace_first_char (text: [all_text](#all_text), letter: char) -> [all_text](#all_text)`


### Function identify_integer

`func identify_integer (num: [all_integers](#all_integers)) -> u8`


### Function identify_float

`func identify_float (num: [all_floats](#all_floats)) -> u8`


### Function identify_text

`func identify_text (text: [all_text](#all_text)) -> u8`


### Function add_one_duplicated

`func add_one_duplicated (num: [duplicated_s32](#duplicated_s32)) -> [duplicated_s32](#duplicated_s32)`


### Function identify_duplicated

`func identify_duplicated (num: [duplicated_s32](#duplicated_s32)) -> u8`


### Function add_one_distinguishable_num

`func add_one_distinguishable_num (num: [distinguishable_num](#distinguishable_num)) -> [distinguishable_num](#distinguishable_num)`


### Function identify_distinguishable_num

`func identify_distinguishable_num (num: [distinguishable_num](#distinguishable_num)) -> u8`

