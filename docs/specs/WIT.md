# The `wit` format specification

## Lexical structure

The `wit` format is a curly-braced-based format where whitespace is optional (but recommended). It is intended to be easily human readable and supports features like comments, multi-line comments, and custom identifiers. A `wit` document is parsed as a unicode string, and when stored in a file is expected to be encoded as UTF-8.

Additionally, wit files must not contain any bidirectional override scalar values, control codes other than newline, carriage return, and horizontal tab, or codepoints that Unicode officially deprecates or strongly discourages.

The current structure of tokens are:

```
token ::= whitespace
        | comment
        | operator
        | keyword
        | identifier
```

Whitespace and comments are ignored when parsing structures defined later.

## Keywords

Certain identifiers are reserved for use in `wit` documents and cannot be used bare as an identifier. These are used to help parse the format, and the list of keywords is still in flux at this time but the current set is:

```
keyword ::= 'type'
    | 'func'
    | 'u8' | 'u16' | 'u32' | 'u64'
    | 's8' | 's16' | 's32' | 's64'
    | 'float32' | 'float64'
    | 'bool'
    | 'char'
    | 'string'
    | 'record'
    | 'flags'
    | 'variant'
    | 'enum'
    | 'union'
    | 'tuple'
    | 'list'
    | 'option'
    | 'result'
    | 'interface'
    | unused-but-reserved

unused-but-reserved ::= 'resource'
    | 'use'
    | 'as'
    | 'from'
    | 'static'
```

## Whitespace

A `whitespace` token in `wit` is a space, a newline, a carriage return, or a tab character:

```
whitespace ::= ' ' | '\n' | '\r' | '\t'
```

## Comments

A `comment` token in wit is either a line comment preceded with `//` which ends at the next newline (`\n`) character or it's a block comment which starts with `/*` and ends with `*/`. Note that block comments are allowed to be nested and their delimiters must be balanced

```
comment ::= '//' character-that-isnt-a-newline*
          | '/*' any-unicode-character* '*/'
```

There is a special type of comment called documentation comment. A doc-comment is either a line comment preceded with `///` which ends at the next newline (`\n`) character or it's a block comment which starts with `/**` and ends with `*/`. Note that block comments are allowed to be nested and their delimiters must be balanced

```
doc-comment ::= '///' character-that-isnt-a-newline*
          | '/**' any-unicode-character* '*/'
```

## Identifiers

Identifiers in wit can be defined with two different forms. The first is a kebab-case identifier:

```
foo: func(bar: u32) -> ()

red-green-blue: func(r: u32, g: u32, b: u32) -> ()
```

This form can't name identifiers which have the same name as wit keywords, so the second form is the same syntax with the same restrictions as the first, but prefixed with '%':

```
%foo: func(%bar: u32) -> ()

%red-green-blue: func(%r: u32, %g: u32, %b: u32) -> ()

// This form also supports identifiers that would otherwise be keywords.
%variant: func(%enum: s32) -> ()
```

## Name resolution

Names in a wit document can only be defined once:

```wit
type foo = u32
type foo = u64  // ERROR: name `foo` already defined
```

Names do not need to be defined before they're used (unlike in C or C++), it's ok to define a type after it's used:

```wit
type foo = bar

record bar {
    age: u32,
}
```

Types, however, **cannot be recursive**:

```
type foo = foo  // ERROR: cannot refer to itself

record bar1 {
    a: bar2,
}

record bar2 {
    a: bar1,  // ERROR: record cannot refer to itself
}
```

## Top-level items

A `wit` document consists of a single interface declared at the top level.

Concretely, the structure of a wit document is:

```
wit-document ::= interface-item
```

## Item: `interface`

Interfaces have a name and a sequence of items and functions.

Specifically interfaces have the structure:

```
interface-item ::= 'interface' id '{' interface-items* '}'

interface-items ::= variant-items
                  | record-item
                  | union-items
                  | flags-items
                  | enum-items
                  | type-item
                  | func-item
                  | use-item
                  | resource-item
```

## Item: `func`

A function has a list of parameters and a single return type of a list of return types.
Both parameters appear as a comma (`,`) separated list wrapped in parentheses (`(`,`)`) where every type must be prefixed with a name. Names and types are separated by colons (`:`).
When multiple returns are specified they must use the same named structure as parameters.

```wit
func mra()
func mrb() -> ()
func mrc(a: u32) -> u32
func mrd() -> (a: u32)
func mre(a, string, b: s64) -> (a: u32, b: float32)
```

Specifically functions have the structure:

```
func-item ::= 'func' id param-list '->' result-list

param-list ::= '(' named-type-list ')'

result-list ::= ty
              | '(' named-type-list ')'

named-type-list ::= nil
                  | named-type ( ',' named-type )*

named-type ::= id ':' ty
```

## Item: `type` (alias)

A `type` statement declares a new named type in the wit document. This name can be later referred to when defining items using this type. This construct is similar to a type alias in other languages.

```wit
type my-awesome-u32 = u32
type my-complicated-tuple = tuple<u32, s32, string>
```

Specifically the structure of this is:

```
type-item ::= 'type' id '=' ty
```

## Item: `record` (bag of named fields)

A `record` statement declares a new named structure with named fields. Records are similar to a `struct` in many languages. Instances of a `record` always have their fields defined.

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

Specifically the structure of this is:

```
record-item ::= 'record' id '{' record-fields '}'

record-fields ::= record-field
                | record-field ',' record-fields?

record-field ::= id ':' ty
```

## Item: `flags` (bag of bools)

A `flags` statement defines a new `record`-like structure where all the fields are booleans. The `flags` type is distinct from `record` in that it is represented as a bit flags representation in the ABI. For the purposes of type-checking, however, it's simply syntactic sugar for a record-of-booleans.

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

Specifically the structure of this is:

```
flags-items ::= 'flags' id '{' flags-fields '}'

flags-fields ::= id
               | id ',' flags-fields?
```

## Item: `variant`  (one of a set of types)

A `variant` statement defines a new type where instances of the type match exactly one of the variants listed for the type. This is similar to a "sum" type in algebraic datatypes (or an `enum` in Rust if you're familiar with it). Variants can be thought of as tagged unions as well.

Each case of a variant can have an optional type associated with it which is present when values have that particular case's tag.

All `variant` type must have at least one case specified.

Specifically the structure of this is:

```
variant-items ::= 'variant' id '{' variant-cases '}'

variant-cases ::= variant-case
                | variant-case ',' variant-cases?

variant-case ::= id
               | id '(' ty ')'
```

## Item: `enum` (variant but with no payload)

An `enum` statement defines a new type which is semantically equivalent to a `variant` where none of the cases have a payload type.

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

Specifically the structure of this is:

```
enum-items ::= 'enum' id '{' enum-cases '}'

enum-cases ::= id
             | id ',' enum-cases?
```

## Item: `union` (variant but with no case names)

A `union` statement defines a new type which is semantically equivalent to a `variant` where all of the cases have a payload type and the case names are numerical.

```wit
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

Specifically the structure of this is:

```
union-items ::= 'union' id '{' union-cases '}'

union-cases ::= ty
              | ty ',' union-cases?
```

## Item: `use` (import)

TODO

## Item: `resource`

TODO

## Types

```
ty ::= 'u8' | 'u16' | 'u32' | 'u64'
     | 's8' | 's16' | 's32' | 's64'
     | 'float32' | 'float64'
     | 'char'
     | 'bool'
     | 'string'
     | tuple
     | list
     | option
     | result
     | id
```

The `tuple` type is semantically equivalent to a `record` with numerical fields, but it frequently can have language-specific meaning so it's provided as a first-class type.

Similarly the `option` and `result` types are semantically equivalent to the variants:

```wit
variant option {
    none,
    some(ty),
}

variant result {
    ok(ok-ty)
    err(err-ty),
}
```

These types are so frequently used and frequently have language-specific meanings though so they're also provided as first-class types.

Finally the last case of a `ty` is simply an `id` which is intended to refer to another type or resource defined in the document.
