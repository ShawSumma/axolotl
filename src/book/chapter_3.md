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
enum Complex: pub {
    Str(String),
    Field1(u32),
    Field2,
    Field3
}
```

### Generics

in the example below, the generic T would be statically checked to ensure that it is Debug, however dynamic dispatch would still be used at runtime to prevent binary bloat, and increased compile times, due to the cost of monomorphizing.

```
fn my_func<T: Debug>(val: &T) {
    echo "{:?}" + val;
}

my_func<&String>("hello world");
my_func<bool>(true);
```