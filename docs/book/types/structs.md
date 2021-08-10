# Structures

### Example Structure
```rust
struct MyStruct<T: Constraint>: pub {
    field1: String,
    field2: u32,
    field3: Complex,
}

// Associated Method
fn assoc_method(object: MyStruct) -> &String {
    &object.field1
}

```

### Explanation

axolotl structures syntax is very similar to rust structs, however some key differences exist, principally the publicity modifier comes after the struct keyword, but before the curly braces, this is done for the sake of simplicity, and consistency. by putting the struct keyword first, it becomes easier to use regex to find structs. (Search the beging of each line for a struct keyword) the other large differences are how associated methods are determined. In rsut an associated function is determined by the existence of the keywords, &self, self, and &mut self, in axolotl however, like in vale, associated functions are determined by the first type that appears in the type signature, as shown above.