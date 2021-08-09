# Types

## Primitives

### Floats
the supported float formats are f32, and f64, large floats may be added at a later date.

### Integers
supported integer types are u8, i8, u32, i32, i64, u64

### Strings
Strings are a builtin type as axolotl is intended to be a higher level scripting language, it made more sense for them to be builtin.

### Enumerations
Composit, or complex enumerations or enums are supported, for example

```rust
enum Complex {
    Str(String),
    Field1(u32),
    Field2,
    Field3
}
```

### Structures

