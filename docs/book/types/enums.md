# Enumerations

Axolotl's enums are another thing inspire by rust, principally they are complex, or composite enumerations. See the example below.

```rust
enum Complex: pub {
    Str(String),
    Field1(u32),
    Field2,
    Field3
}
```

In this definition, the Str and Field1 varients are composites, and store the types String, and u32 respectively.