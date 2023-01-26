store = 4 + 73 = 77
classic = 7 + 39 = 46
web = 10 + 77 = 87
total = 23 + 197 = 220

## Alignment

Each value type is assigned an alignment which is used by subsequent Canonical ABI definitions. Presenting the definition of alignment piecewise, we start with the top-level case analysis:

| type               | alignment             |
| ------------------ | --------------------- |
| `bool`             | 1                     |
| `s8`/`u8`          | 1                     |
| `s16`/`u16`        | 2                     |
| `s32`/`u32`        | 4                     |
| `s64`/`u64`        | 8                     |
| `s128`/`u128`      | 16                    |
| `float32`          | 4                     |
| `float64`          | 8                     |
| `char`             | 4                     |
| `string`/`list<t>` |                       |
| `record<fields>`   | max_alignment(fields) |
| `variant<cases>`   | max_alignment(cases)  |
| `flags<fields>`    | either 1, 2, 4, 8, 16 |

## size

| type               | size (bytes           |
| ------------------ | --------------------- |
| `bool`             | 1                     |
| `s8`/`u8`          | 1                     |
| `s16`/`u16`        | 2                     |
| `s32`/`u32`        | 4                     |
| `s64`/`u64`        | 8                     |
| `s128`/`u128`      | 16                    |
| `float32`          | 4                     |
| `float64`          | 8                     |
| `char`             | 4                     |
| `string`/`list<t>` |                       |
| `record<fields>`   | max_alignment(fields) |
| `variant<cases>`   | max_alignment(cases)  |
| `flags<fields>`    | either 1, 2, 4, 8, 16 |

## void

## unsigned integers

little endian

```
     +--------+
u8   | 1 byte |
     +--------+ 
     +--------+--------+
u16  | 2 bytes         |
     +--------+--------+ 
     +--------+--------+--------+--------+
u32  | 4 bytes                           |
     +--------+--------+--------+--------+
     +--------+--------+--------+--------+--------+--------+--------+--------+
u64  | 8 bytes                                                               |
     +--------+--------+--------+--------+--------+--------+--------+--------+
     +--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+
u128 | 16 bytes                                                                                                                                      |
     +--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+
```

## signed integers

little endian, 2s complement

```
     +--------+
s8   | 1 byte |
     +--------+ 
     +--------+--------+
s16  | 2 bytes         |
     +--------+--------+ 
     +--------+--------+--------+--------+
s32  | 4 bytes                           |
     +--------+--------+--------+--------+
     +--------+--------+--------+--------+--------+--------+--------+--------+
s64  | 8 bytes                                                               |
     +--------+--------+--------+--------+--------+--------+--------+--------+
     +--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+
s128 | 16 bytes                                                                                                                                      |
     +--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+--------+
```

## floats

“binary32” or “binary64” IEEE 754-2008

```
         +--------+--------+--------+--------+
float32  | 4 bytes                           |
         +--------+--------+--------+--------+
         +--------+--------+--------+--------+--------+--------+--------+--------+
float64  | 8 bytes                                                               |
         +--------+--------+--------+--------+--------+--------+--------+--------+
```

## char

UTF8 codepoint

```
       +--------+
char   | 1 byte |
       +--------+ 
```

## bool

true = 1, false = 0

```
       +--------+
bool   | 1 byte |
       +--------+ 
```

## list<t>

```
          +-------------------------------+--------+--------+--------+--------+--------+--------+--------+--------+
list<t>   | content (n * sizeof(t) bytes) | RelPtr to the content (u32)       | Length (u32)                      |
          +-------------------------------+--------+--------+--------+--------+--------+--------+--------+--------+
```

## string

same as `list<u8>` or inline repr for strings shorter than 7 chars

```
               +--------+--------+--------+--------+--------+--------+--------+-------------+
               | length (u32)                      | content (length bytes)

                 +--------+--------+--------+--------+--------+--------+--------+-------------+
string (inline)  | content (7 bytes)                                            | length (u8) |
                 +--------+--------+--------+--------+--------+--------+--------+-------------+
                 +--------------------------+--------+--------+--------+--------+--------+--------+--------+--------+
string (default) | content (length bytes)   | RelPtr to the content (u32)       | Length (u32)                      |
                 +--------------------------+--------+--------+--------+--------+--------+--------+--------+--------+
```

## tuple

same as record where the keys are the tuple fields index

```
tuple<a,b,c> = record {
    0: a,
    1: b,
    2: c
}
```

## record

```
       +-----------------+-------------------+-------------------+-------------------+ 
record | field (n bytes) | field 2 (n bytes) | ...               | field x (n bytes) |
       +-----------------+-------------------+-------------------+-------------------+ 
```

```rust
#[derive(Debug, PartialEq)]
struct Foo {
     a: u8,
     b: u64,
     c: String,
}

impl Readable for Foo {
     fn read_from(read: &mut impl io::Read) -> io::Result<Self> {
          Ok(Self {
          a: Readable::read_from(read)?,
          b: Readable::read_from(read)?,
          c: Readable::read_from(read)?,
          })
     }
}
```

## variant

Prefixed with a `tag` of u8, u16, u32, u64, or u128, choosing the smallest possible type that can represent all of the cases.
followed the by the content of the type.

```
        +---------------+-------------------+
variant | tag (n bytes) | content (n bytes) |
        ----------------+-------------------+
```

```rust
#[derive(Debug, PartialEq)]
pub enum U1 {
     U32(u32),
     F32(f32),
}

impl Readable for U1 {
     fn read_from(read: &mut impl io::Read) -> io::Result<Self> {
          let tag = u8::read_from(read)?;

          match tag {
               0 => Ok(Self::U32(Readable::read_from(read)?)),
               1 => Ok(Self::F32(Readable::read_from(read)?)),
               _ => panic!(),
          }
     }
}
```

## enum

Represented by either u8, u16, u32, u64, or u128, choosing the smallest possible type that can represent all of the cases.

```
     +-----------------+
enum | tag (n bytes)   |
     +-----------------+
```

## union

same as `variant`

## option

## result

## flags

A bag-of-bools, represented by a bitfield of size u8, u16, u32, u64, or u128, choosing the smallest possible type that can represent all of the fields.

```
      +----------------------+
flags | bitfield (n bytes)   |
      +----------------------+
```

```rust
bitflags::bitflags! {
     pub struct Flag4: u8 {
          const B0 = 1 << 0;
          const B1 = 1 << 1;
          const B2 = 1 << 2;
          const B3 = 1 << 3;
     }
}

impl Readable for Flag {
     fn read_from(read: &mut impl io::Read) -> io::Result<Self> {
          let bits = Readable::read_from(read)?;
          Self::from_bits(bits).ok_or(io::ErrorKind::InvalidData.into())
     }
}
```


## resource

same as `u64`