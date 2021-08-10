# Commands

One of the features that makes Axolotl fairly unique is its system of commands. With a desire to create an intuitive scripting language, a command is a way of easily interoperating with different programming languages, principally through the use of the systems command interface. for the sake of efficiency the builtins or commands feature allows system commands to easily call native code including rust, and axolotl code. 

An interesting thought experiment with this language is how to denote what should and shouldn't be executed on the command line. none the less to define a builtin for the shell one can do

```
cmd mybuiltin(args: Array<str>) -> ExecResult {
    echo $args[1]
}

echo "hello world"
```

which can then (for now at least) be invoked as 
```
mybuiltin "hello world"
```

all builtins return an ExecResult who's signature is

```rust
enum ExecResult: pub {
    // where String is stdout
    Ok(String)
    // return code, and stderr
    Err((i32, e)),
}
```