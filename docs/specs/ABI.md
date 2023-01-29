# The `tauri-bindgen` application binary interface

## Scalars

### Integers

The unsigned integer types consist of:

| Type  | Length | Minimum | Maximum |
| ----- | ------ | ------- | ------- |
| `u8`  | 8-bit  | 0       | 28-1    |
| `u16` | 16-bit | 0       | 216-1   |
| `u32` | 32-bit | 0       | 232-1   |
| `u64` | 64-bit | 0       | 264-1   |

Signed numbers are stored using [two’s complement][twos-complement] representation and consist of:

| Type  | Length | Minimum           | Maximum          |
| ----- | ------ | ----------------- | ---------------- |
| `s8`  | 8-bit  | -(2<sup>7</sup>)  | 2<sup>7</sup>-1  |
| `s16` | 16-bit | -(2<sup>15</sup>) | 2<sup>15</sup>-1 |
| `s32` | 32-bit | -(2<sup>31</sup>) | 2<sup>31</sup>-1 |
| `s64` | 64-bit | -(2<sup>63</sup>) | 2<sup>63</sup>-1 |

### Floating-point Numbers

Floating-point numbers are represented according to the IEEE-754 standard. The
`f32` type is a 32-bit single-precision float, and `f64` a 64-bit double precision float.

### `char`

A value of type char is a [Unicode Scalar Value][unicode-scalar-value] (i.e. a code point that is not a surrogate), represented as a 32-bit unsigned word in the 0x0000 to 0xD7FF or 0xE000 to 0x10FFFF range.

### `bool`

A bool is 1 byte in size and has two possible values `0` and `1` where `0` represents `false` and `1` represents `true`. All other values must be rejected.  

## Compound Types

### `record`

A `record`'s fields are all laid out next to each other.

```
       +-----------------+-------------------+-------------------+-------------------+ 
record | field (n bytes) | field 2 (n bytes) | ...               | field x (n bytes) |
       +-----------------+-------------------+-------------------+-------------------+ 
```

### `tuple`

A tuples binary representation is the same as that of `struct`.

### `variant`

A variants binary representation is the representation of the case's payload prefixed with a *tag* of type of `u8`, `u16`, `u32`, or `u64` choosing the smallest possible type that can represent all variants.

```
        +---------------+-------------------+
variant | tag (n bytes) | content (n bytes) |
        ----------------+-------------------+
```

### `enum`

An enums binary representation is the same as that of a variant without case payloads. I.e. just the *tag*.

```
     +-----------------+
enum | tag (n bytes)   |
     +-----------------+
```

### `union`

A unions binary representation is the same as that of `variant`.

### `option`

An option's binary representation is the same it's desugared variant representation.

```
variant {
    none,
    some<t>
}
```

### `result`

An results's binary representation is the same it's desugared variant representation.

```
variant {
    ok<t>,
    err<e>
}
```

## `flags`

A bag-of-bools, represented by a bitfield of size u8, u16, u32, u64, or u128, choosing the smallest possible type that can represent all of the fields.

```
      +----------------------+
flags | bitfield (n bytes)   |
      +----------------------+
```

## list

A lists binary representation is the length of the list encoded as a `u32` followed by `n` bits of content where `n = length * sizeof(t)`

```
        +--------+--------+--------+--------+--------+--------+--------+--------+--------+
list<t> | length (u32)                      | content (length * sizeof(t) bytes)         |
        +--------+--------+--------+--------+--------+--------+--------+--------+--------+
```

## string

A strings binary representation is the same as `list<u8>`

[twos-complement]: https://en.wikipedia.org/wiki/Two%27s_complement

## resource

same as `u64`