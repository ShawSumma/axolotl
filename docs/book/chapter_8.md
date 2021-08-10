# Syntax Examples


## Command + Interface implementation
```
let contents: Array<str> = $(ls)
echo "hello world"

interface Collection<T: Debug> {
    fn printall(self: impl Collection<T>);
}

fn printall(self: Array<str>)

```

### Lets break this down

```
let contents = $(ls)
```

This code snippet, declares a variable using the let binding, this variable is constructed from the value written to stdout by the ls command.

```
echo "hello world"
```
executes the command hello world, the exact parsing structure is to be determined, but will likely need to be at least partially resolved at runtime.

```
interface Collection<T: Debug> {
    fn printall(self: impl Collection<T>);
}
```
this line of code declares an interface, the interface requires a generic parameter who's type must be debugable. Specifically the declared method, printall takes in an object of Collection, and is automatically an associated method on whatever type ends up impplementing this interface.

