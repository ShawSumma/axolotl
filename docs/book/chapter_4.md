# Functions
functions use the C style of scope definition principally () and {} to denote the arguments, and body to functions, this was chosen instead of POSIX fi, done, if, end, etc because it allows one notation for all scopes.

## Associated Functions

an Associated function, that is to say a function that can be called with the . operator is any function who's first argument is of the type to which the function is associated, for example

```rust
fn my_assoc_fn(self: String) -> String;
```

is implicitly an associated function that can be called on the String type, supposing the function is imported into the current namespace.

## Functions as First Class Citizens
Functions can be treated as first class citizens, and can be passed around as such.