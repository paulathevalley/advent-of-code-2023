# advent of code 2023

according to the [docs](https://doc.rust-lang.org/stable/book/), I am a rust programmer!

## check code

```bash
$ cargo check 
```

## build 

```bash
$ cargo run
```

when your build is ready for release:

```bash
$ cargo build --release
```

## my reading notes

### chapter 1

without cargo, you would first compile:

```bash
$ rustc ./main.rs
``` 

which should generate a binary executable:

```bash
$ ls
main.rs   main
```

that you can run (on any machine, without needing to install rust): 

```bash
$ ./main 
hello!
```

neat.

with cargo, you would build:

```bash
$ cargo build 
```

which generates an executable, `./target/debug/advent-of-code-2023`

```bash
$ ./target/debug/advent-of-code-2023
hello!
```

to build and run the executable in a single command, use: 

```bash 
$ cargo run
```

### chapter 3

#### variables.

```rust 
  let x = 5; // this variable is immutable 
  // x = 6; // the compiler will error
  mut y = 5; // this variable is mutable
  println!("The value of y is: {y}");
  y = 6; // okie dokies
  println!("The value of y is: {y}");
```

constants are always immutable and must have a type annotation.
constants can be declared in any scope, including the global scope.
rust's naming convention for constants is to use all uppercase with underscores between words. 

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

#### shadowing.

```rust
let x = 5; 
let x = x + 1; // 'shadowed' (or overshadows) the first variable
{
  // inner scope can 'temporarily' shadow
  let x = x * 12; 
  println!(x) // prints 12
}
println!(x) // prints 6
```

`let` variables can change type:

```rust
let spaces = "     "; // variable is a string type
let spaces = spaces.len(); // new variable is a number type
```

`mut` variables can change value, but they cannot change type.

#### data types.

rust is a _statically typed_ language, which means it must know all the types of all variables at compile time. so omniscient.

scalar types: integer, floating-point numbers, booleans, and characters

'signed' means it needs a positive or negative sign '-1' vs '+1' 
'unsigned' means it will only ever be positive 

booleans

```rs
let t = true;

let f: bool = false; // with explicit type annotation
```

characters 

> Rust’s `char` type is the language’s most primitive alphabetic type.

```rs
let c = 'z';
let z: char = 'ℤ'; // with explicit type annotation
let heart_eyed_cat = '😻';
```

`char` uses single quotes, as opposed to string literals, which use double quotes. 
`char` represents a Unicode Scalar Value.

##### compound types.

compound types can group multiple types into one type. 

a tuple is a group of various types of a fixed length. they cannot change size. 

```rs
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup; // destructure to get values of a tuple
```

> The tuple without any values has a special name, _unit_. This value and its corresponding type are both written `()` and represent an empty value or an empty return type. Expressions implicitly return the unit value if they don’t return any other value.

an array is a group of a single type of a fixed length.

a vector is similar to array and is allowed to grow or shrink in size.

```rs
let a: [i32; 5] = [1, 2, 3, 4, 5]; 
```
`i32` is the type of each element. the array has `5` elements.

```rs
let a = [3; 5];
```
`a` will contain 5 elements that will all be set to the value `3`
it is the same as writing `let a = [3, 3, 3, 3, 3];`

a `runtime` error can happen when the compiler can’t possibly know the value a user will enter when they run the code later. 

#### functions.

the `fn` keyword declares new functions 

use _snake case_ as the conventional style for function and variable names, e.g. `fn another_function()`

> Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.

functions have _parameters_, special variables that are part of the function’s signature. technically, the concrete values are called _arguments_ although these two terms are used interchangeably in casual conversation.

```rs
fn main() {
  another_function(5); // has argument `5`
}

fn another_function(x: i32) { // has one parameter named `x`
  println!("The value of x is: {x}");
}
```

you _must_ delcare the type of each parameter in function signatures.



