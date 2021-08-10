# Axolotl Errors

To start with the std error types of axolotl will support partial erroneous states, rather then being simply success or failure. Similar to rust axolotl will support return based error deferal, but will also provide a more interactive backtrace. While rust does provide backtraces, they are often controlled through environment variables, and can easily get messy, as there isn't much control over how much, or what is printed. Axolotl will allow a integer to be used to show how many stack frames should be shown, and what variables values where at each frame. This backtrace will have an option to be run interactively when interpreted, or simply printed (depending on release state). This will be accomplished by stack switching at runtime so the programs state can be preserved, supposing that the errors aren't defered. This information will be supplied as first class information, for the perposes of the interactive mode, this means that variables can be manually set. (an embedded debugger mode)

```rust
// panic handler signature

struct Frame: pub {
    function: String,
    args: Vec<Value>,
}

fn handle(trace: StackTrace) {

}
```