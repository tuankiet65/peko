# peko-rlp

`peko-rlp` implements the RLP (Recursive Length Prefix) serialization format on top of
[`serde`](https://serde.rs/).

## RLP
RLP is formally defined in the Ethereum Yellow Paper, Appendix B. Recursive Length Prefix.

RLP is capable of serializing one item, which can be:
* A byte array of arbitrary length
* A sequence of zero or more RLP-encoded items (a byte array or another sequence)

Unsigned integers are also supported by encoding them as big-endian byte arrays.

## Usage

### Serializing
Use `peko_rlp::to_bytes()` to serialize values into RLP-encoded byte array:
```rust
```

### Deserializing
Deserialization can be done through the `Item` system, or the serde's `Deserialize` system.

`peko_rlp::parser::actual_parser()` parses an RLP-encoded byte array into a recursive `Item`:
```rust
```

## Implementation details

### Unsupported types
`peko-rlp` currently does not support serializing the following types:

* Signed integers (`i8`, `i16`, ...)
* Floats (`f32`, `f64`)
* Option (`None`, `Some`)
* `Unit`, `UnitVariant`, `UnitStruct`
* `NewTypeStruct`, `NewTypeVariant`
* `TupleStruct, TupleVariant`
* `StructVariant`

Signed integers and floats will never be supported, since the RLP specification does not
specify a canonical encoding for them. Other types might be supported in the future if needed.

### Supported types.
The following types are supported by `peko-rlp`.

### Byte array
Byte array is natively supported by RLP.

### String
Strings are serialized as UTF-8 byte arrays.

### Sequence and tuple
Sequences and tuples are encoded as RLP sequences.

### Map
Maps are encoded as a RLP sequence containing multiple sub-sequences, each sequence containing `Kn`,
the RLP-encoded key, and `Vn`, the RLP-encoded value.

```
[[K1, V1], [K2, V2], ..., [Kn, Vn]]
```

### Struct
Structs are encoded as an RLP sequence, where each item in the sequence corresponds to a struct field.
For example:
```rust
struct Student {
    name: String,
   class: String,
    school_year: int
}
```

is transformed into
```
[name, class, school_year]
```

and RLP-encoded.

## Credits

The overall design and API is heavily inspired by [`serde_json`](https://docs.serde.rs/serde_json/)
