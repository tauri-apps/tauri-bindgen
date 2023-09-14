# The `wit` format

`wit` is a simple interface definition language (IDL) that helps you define a contract between sides of an IPC bridge. It defines a simple yet comprehensive common type system that lets you efficiently declare all functions a *Guest* can expect to call as well as their parameters and possible return types.

This language is designed to be easy to read and write for humans, as well as to cleanly map into a variety of languages.

In this guide we will introduce and explain all elements of the `wit` type system, beginning with the primitive types:

## Primitives

`wit` has four primary primary types: integers, floating-point numbers, Booleans, and characters. You may recognize these from other programming languages.

### Integers

An integer is a number without a fractional component. Integers come in two variants: *signed* and *unsigned* and have an explicit size. Signed and unsigned refer to whether it’s possible for the number to be negative. Unsigned numbers can only store positive integers including zero (i.e. they don't have e sign) while Signed numbers can represent any integer positive or negative.

The size of integer dictates the range of numbers that can be represented. Each signed variant can store numbers from -(2<sup>n - 1</sup>) to 2<sup>n -
1</sup> - 1 inclusive, where *n* is the number of bits that variant uses. So an `s8` can store numbers from -(2<sup>7</sup>) to 2<sup>7</sup> - 1, which equals -128 to 127. Unsigned variants can store numbers from 0 to 2<sup>n</sup> - 1, so a `u8` can store numbers from 0 to 2<sup>8</sup> - 1, which equals 0 to 255.

<figure>
    
| Length | Signed | Unsigned |
| ------ | ------ | -------- |
| 8-bit  | `s8`   | `u8`     |
| 16-bit | `s16`  | `u16`    |
| 32-bit | `s32`  | `u32`    |
| 64-bit | `s64`  | `u64`    |

<figcaption>Table TODO: Integer Types in WIT</figcaption>
</figure>

Here's an example of integers in w `wit` document:

```wit
type age = u8

type milliseconds = u64
```

### Floating-Point Types

Floating point numbers are numbers with decimal points. The floating point types defined in `wit` are `float32` and `float64` which are 32 bits and 64 bits in size, respectively. All floating-point types are signed.

```wit
type very-precise-measurement = float64
```

### The Boolean Type

A boolean type has one of two possible values: `true` or `false`. Booleans are one byte in size. The Boolean type in `wit` is specified using the keyword `bool`.

### The Character Type

The character type is the most basic textual type in `wit`. It is four bytes in size and represents a [Unicode Scalar Value][unicode-scalar-value]. Unicode Scalar Values can represent a lot more than just ASCII: Chinese, Japanese, and Korean characters; emoji; and accented letters are all valid char values.

## Compound Types

*Compound types* can group multiple values into one type. WIT has a two
primitive compound types: tuples and lists.

### Tuple

A tuple groups a number of various types into one compound type. Tuples have a fixed length, and a fixed order.

```wit
tuple<u32, char, bool>
```

### List

Another collection type is the *list*. Unlike a tuple, every element of a list must have the same type. Unlike tuples however, lists can be arbitrary in length.

```wit
list<u8>
// or something more interesting
list<string>
```

## Option

A very common scenario when doing computation is the fact that functions may return *something* or *nothing*. For example querying a database for an entry may result in that entry being found, **or not**. The `option` type gives you a handy way to indicate this.

```wit
// a user may have stored their email address or not, so we wrap it in an option
option<string>
```

## Result

A *Result* gives you a way to indicate that a computation **may fail**. Similar to `option` it has two variants to express this: *Ok* and *Err*. The Ok variant indicates the operation was successful, and inside Ok is the successfully generated value. The Err variant means the operation failed, and Err should contain information about how or why the operation failed.

```wit
// reading from a file may fail, so in case of failure we return an error code
result<string, u32>
```

See the [Section about Enums](#enum) to see how you can make the error type more helpful and descriptive.

## Strings

A string is a sequence of characters. Strings are represented as a sequence of bytes, similar to `list<u8>`, but all bytes are guaranteed to be valid UTF8.

```wit
type name = string

func greet(name: string) -> string
```

# The Interface

At the top-level of each `wit` document lives the `interface` definition, file must contain exactly one such definition. 
The name you give to an interface will dictate the name of the generated module and printed debug output.

An interface may contain function declarations and type definitions. The order of declaration doesn't matter so you are free to define types after you have used them for example.

```wit
interface empty {}
```

## Alias

Using the `type` keyword you can introduce new named types. These names can later be reused in other type definitions.

```wit
type my-awesome-u32 = u32
type my-complicated-tuple = tuple<u32, s32, string>
```

## Record (bag of named fields)

A `record` statement declares a new named structure with named fields. Records are similar to a `struct` in Rust or `interface` in TypeScript.

```wit
record pair {
    x: u32,
    y: u32,
}

record person {
    name: string,
    age: u32,
    has-lego-action-figure: bool,
}
```

## Flags (bag-of-bools)

Flags can be thought of as a record where all fields are `bool`s. Since it is much more efficient to represent these types as bit flags where each bit in a number refers to a certain field than full structs `flags` is special-cased. 

You can also think of `flags` as an `enum` that holds possibly multiple cases at the same time instead of just one.

```wit
flags properties {
    lego,
    marvel-superhero,
    supervillan,
}

// type-wise equivalent to:
//
// record properties {
//     lego: bool,
//     marvel-superhero: bool,
//     supervillan: bool,
// }
```

## Variant

A `variant` defines a new type where instances of the type match exactly one of the variants listed for the type. This is similar to a "sum" type in algebraic datatypes (or an `enum` in Rust if you're familiar with it). Variants can be thought of as tagged unions as well.

Each case of a variant can have an optional type associated with it which is present when values have that particular case's tag.

```wit
variant filter {
    all,
    none,
    some(list<string>),
}
```

## Enum

Enums allow you to define a set of mutually exclusive names. This is commonly used to express a set of named constants. An `enum` is semantically equivalent to a `variant` where none of the cases have a payload type.

```wit
enum color {
    red,
    green,
    blue,
    yellow,
    other,
}

// type-wise equivalent to:
//
// variant color {
//     red,
//     green,
//     blue,
//     yellow,
//     other,
// }
```

Enums are a great type for declaring better error types. In the following example the function returns C-style error codes when the operation failed:

```wit
func read-file(path: string) -> result<list<u8>, u32>
```

Thats not very readable and user-friendly though, but here is where enums really shine. We can use an enum to define all possible errors that can happen, so callers can handle them appropriately. Using enums means we can give each case a human-readable name and even add some doc-comments to explain whats going on:

```wit
// declare all the ways that `read-to-string` may fail
enum error {
    /// The file couldn't be found
    not-found,
    /// You don't have the right permissions to access this
    permission-denied,
    /// It's not a file but a directory!
    is-a-directory,
    /// The file is too large
    file-too-large,
    /// The file contains bytes that aren't valid utf8
    utf8
}

func read-to-string(path: string) -> result<list<u8>, error>
```

## Union

A `union` type describes a value that can be one of several types. This can be used to express a function that takes a set of different types as arguments. A union is semantically equivalent to a `variant` where all of the cases have a payload type and the case names are numerical.

```
union configuration {
    string,
    list<string>,
}

// type-wise equivalent to:
//
// variant configuration {
//     0(string),
//     1(list<string>),
// }
```

## Function

Functions are at the heart of every interface definition, they declare what computation the *Guest* may request from the *Host*. Functions have *Parameters* and *Results*. As a `wit` document only defines a contract between two sides of an IPC boundary the actual implementation of each function is left for the *Host*.

```wit
func greet(name: string) -> string

// can be made more descriptive using named returns:
func greet(name: string) -> (greeting: string)

// a simple user record
record user {
    name: string,
    age: u8
}

// looks up a user by name, this might not find anything so we return an option
func find-user(name: string) -> option<user>

// declare all the ways that `access-file` may fail
enum io-error {
    not-found,
    permission-denied,
    is-a-directory,
    file-too-large
}

// a computation that may fail:
func access-file(path: string) -> result<list<u8>, io-error>
```

> **Note: Multi-return**
>
> Functions in WIT have a feature called *Multi-return* which means they can return more than one value. 
> This is a concept that not many languages have, so multi-return get's mapped to the closest language concept applicable. 
> For example `a() -> (a: u32, b: u64)` maps to a function returning a tuple in Rust and a function returning an array in JavaScript and TypeScript.

## Resource

TODO

## Imports

TODO

[unicode-scalar-value]: https://unicode.org/glossary/#unicode_scalar_value