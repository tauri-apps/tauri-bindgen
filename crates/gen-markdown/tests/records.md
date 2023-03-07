# records



## Type definitions

## Struct aggregates



### Fields

#### a: `[scalars](#scalars)`

#### b: `u32`

#### c: `[empty](#empty)`

#### d: `string`

#### e: `[really_flags](#really_flags)`


## Struct empty



### Fields


## Alias int_typedef

`s32`


## Struct really_flags

A record that is really just flags
All of the fields are bool

### Fields

#### a: `bool`

#### b: `bool`

#### c: `bool`

#### d: `bool`

#### e: `bool`

#### f: `bool`

#### g: `bool`

#### h: `bool`

#### i: `bool`


## Struct scalars

A record containing two scalar fields
that both have the same type

### Fields

#### a: `u32`
The first field, named a
#### b: `u32`
The second field, named b

## Alias tuple_typedef2

`tuple<[int_typedef](#int_typedef)>`



## Functions

### Function tuple_arg

`func tuple_arg (x: tuple<char, u32>)`


### Function tuple_result

`func tuple_result () -> tuple<char, u32>`


### Function empty_arg

`func empty_arg (x: [empty](#empty))`


### Function empty_result

`func empty_result () -> [empty](#empty)`


### Function scalar_arg

`func scalar_arg (x: [scalars](#scalars))`


### Function scalar_result

`func scalar_result () -> [scalars](#scalars)`


### Function flags_arg

`func flags_arg (x: [really_flags](#really_flags))`


### Function flags_result

`func flags_result () -> [really_flags](#really_flags)`


### Function aggregate_arg

`func aggregate_arg (x: [aggregates](#aggregates))`


### Function aggregate_result

`func aggregate_result () -> [aggregates](#aggregates)`


### Function typedef_inout

`func typedef_inout (e: [tuple_typedef2](#tuple_typedef2)) -> s32`

