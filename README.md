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

#### [data types](https://doc.rust-lang.org/stable/book/ch03-02-data-types.html).

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

#### [functions](https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html).

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

Rust is an expression-based language. 

> **statements** are instructions that perform some action and do not return a value. 
> **expressions** evaluate to a resultant value. 

```rs
fn main() { // function definitions are also statements
  let y = 6; // statement
}
```

the math operation, `5 + 6`, is an expression that evalutes to the value `11`.

calling a function is an expression. 
calling a macro is an expression. 

a new scope block with curly brackets is an expression, for example:
```rs
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}"); // The value of y is: 4
}
```

this expression:
```rs
{
    let x = 3;
    x + 1 // note: NO semicolon!
}
```
is a block that evaluates to `4`.

NOTE: `x + 1` does _not_ have a semicolon at the end. 
> Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.

functions can return values to the code that calls them, and we must declare their type after an arrow (`->`). 
> In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.
you can return early using the `return` keyword and specifying a value, but most return the last expression implicity. 

i personally love `return` keywords because it lets me know when the fn exits and that it explicitly returns a value when it exits. a missing semicolon does this now. that's gonna be easy for me to miss. gratefully, the compiler will give us useful error messages.

```rs
fn five() -> i32 { // no parameters
    5 // returns the value `5`
}

fn main() {
    let x = five();

    println!("The value of x is: {x}"); // The value of x is: 5
}
```

#### [control flow](https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html).

```rs
fn main() {
    let number = 3;

    if number < 5 { // if, followed by a condition
        println!("condition was true"); 
    } else { // execute this if and only if the condition above evaluates to false
        println!("condition was false");
    }
}
```

all `if` expressions start with the keyword `if`
Blocks of code associated with the conditions in `if` expressions are sometimes called _arms_.

optionally, we can include an `else` expression to give the program an alternative block of code to execute. 

the condition in this code _must_ be a `bool`.
```rs
fn main() {
    let number = 3;

    if number { // this will error: expected `bool`, found integer
        println!("number was three");
    }
}
```

> Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean.

since `if` is an expression that evaluates to a value, we can use it on the right side of a `let` statement. 

```rs
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}"); // The value of number is: 5
}
```

Note: numbers by themselves are expressions. 

Both _arms_ of the `if` expression must return the same type, in this case, `i32`. 

```rs
let number = if condition { 5 } else { "six" }; // will error, because variables must have a single type
```

The `loop` keyword will execute a block over and over again forever.
Use the `break` keyword to tell the program when to stop executing the loop. 
The `continue` keyword in a loop tells the program to skip over any remaining code in this iteration and go to the next iteration. 

One of the uses of `loop` is to retry an operation you know might fail. 

return values from loops 

```rs
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // use the `break` keyword to stop the loop. the loop returns the value after the `break` expression.
        }
    }; // use a semicolon to end the statement that assigns the value to `result`

    println!("The result is {result}"); // The result is 20
}
```

if you have loops within loops, the `break` and `continue` keywords apply to the innermost loop.
you can name or _label_ a loop to indicate the keywords should apply to that loop; loop labels must begin with a single quote.

```rs
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // will exit the inner loop only
            }
            if count == 2 {
                break 'counting_up; // will exit the outer loop 
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

> The outer loop has the label `'counting_up`, and it will count up from 0 to 2. The inner loop without a label counts down from 10 to 9. The first `break` that doesn’t specify a label will exit the inner loop only. The `break 'counting_up;` statement will exit the outer loop. 

```
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.58s
     Running `target/debug/loops`
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
```

`while` keyword
```rs
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```
> This construct eliminates a lot of nesting that would be necessary if you used `loop`, `if`, `else`, and `break`, and it’s clearer. While a condition evaluates to `true`, the code runs; otherwise, it exits the loop.



