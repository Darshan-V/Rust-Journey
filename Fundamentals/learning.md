### Cargo

- cargo is a build system and package manager for rust
- cargo new <package_name>
- This file is in the TOML (Tom’s Obvious, Minimal Language) format, which is Cargo’s configuration format.

#### Building and Running a Cargo Project

- cargo build
- cargo run
- cargo check ->This command quickly checks your code to make sure it compiles but doesn’t produce an executable

#### Rust Macros

The term macro refers to a family of features in Rust: declarative macros with macro_rules! and three kinds of procedural macros:

- Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
- Attribute-like macros that define custom attributes usable on any item
- Function-like macros that look like function calls but operate on the tokens specified as their argument

> **Fundamentally, macros are a way of writing code that writes other code, which is known as metaprogramming.**

- Metaprogramming is useful for reducing the amount of code you have to write and maintain, which is also one of the roles of functions.

#### Trait

A type’s behavior consists of the methods we can call on that type. Different types share the same behavior if we can call the same methods on all of those types. Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

#### Bindings

- **let** declare a variable in rust
- by default variables are immutable in rust, you can't change them, if you wish to mutate it, you have to explicityle specify it by using **mut** keyword
- ```rs
  let mut num = 5
  num = 6
  ```
- Another piece of data that’s immutable, (so can’t change) is a constant. Declared with the const keyword.
- You aren’t allowed to use **mut** on constants. No mutating, ever.
- The type of a constant must be declared, whereas the type of a variable may be declared.
- Constants can only be set to a constant expression, not to the result of a function call or anything that could only be determined at runtime.

> tldr;
>
> > Compile-time and Runtime are the two programming terms used in the software development. Compile-time is the time at which the source code is converted into an executable code while the run time is the time at which the executable code is started running

#### Variables and Mutability
- By default rust variables are immutable
```
let x = 5 
x = 6 
```
the above code throws an error at Compile-time, saying cannot assign twice to immutable variables

- If you want a mutable variable you can use __mut__ keyword
```
let mut x = 5 
x = 7```
the above statement is allowed in rust
#### Shadowing

### Datatypes
Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so it knows how to work with that data.
 - Scalar and Compound types
  - Scalar type represents a single value,
    Rust has 4 major primary types [integers,floating-point,characters,boolean]
  - ##### integers
    - An integer is a number without a fractional component
    |  Length | Signed | Unsigned |
    |:-------:|:------:|:--------:|
    | 8-bit   | i8     | u8       |
    | 16-bit  | i16    | u16      |
    | 32-bit  | i32    | u32      |
    | 64-bit  | i64    | u64      |
    | 128-bit | i128   | u128     |
    | arch    | isize  | usize    |

  - floating-point : f32 f64
  - boolean :bool
  - char :char

* Compount types
 - Tuple type 
 - Array type

  - Typle type example 
  ```
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}```
The variable tup binds to the entire tuple because a tuple is considered a single compound element

* Array type example
 ```
let a: [i32; 5] = [1, 2, 3, 4, 5];

```
Here, i32 is the type of each element. After the semicolon, the number 5 indicates the array contains five elements.

## Inner Development Loop
- Make a change
- Compile the app
- Run tests
- Run the app

#### Compilation speed is a bit slow in the rust -> how to mitigate the issue:
- Faster Linking - lld, a linker developed by the LLVM Project
-```“# .cargo/config.toml```

# On Windows 
 ```
 cargo install -f cargo-binutils
 rustup component add llvm-tools-preview
 ```
[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.x86_64-pc-windows-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

# On Linux:
# - Ubuntu, `sudo apt-get install lld clang`
# - Arch, `sudo pacman -S lld clang`
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]

# On MacOS, `brew install llvm` and follow steps in `brew info llvm`
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/opt/homebrew/opt/llvm/bin/ld64.lld"]”

```
- Cargo watch 

“cargo install cargo-watch”
```

- “```cargo-watch``` monitors your source code to trigger commands every time a file changes.
  For example:
    cargo watch -x check
    will run cargo check after every code change.
    
This reduces the perceived compilation time:
  - you are still in your IDE, re-reading the code change you just made;
  - cargo-watch, in the meantime, has already kick-started the compilation process;
  - once you switch to your terminal, the compiler is already halfway through!”

* Continuous integration: CI pipeline

## Thread Pool
- A thread pool is a group of pre-instantiated, 
idle threads which stand ready to be given work. 
These are preferred over instantiating new threads for each
task when there is a large number of short tasks to be done rather than
a small number of long ones.


## Enums
##### Options enum
- In Rust, both Option and Result are enums that are 
commonly used to handle optional and error-prone situations, respectively.
```  rust
enum Option<T> {
  Some(T),
  None,
  }
```
- It represents either a value (Some(T)) or no value (None). This is useful
when a computation might return a value or might fail to produce a value.
- eg
  ``` rust
  // An Option that can either contain an integer or be None
  let some_value: Option<i32> = Some(42);
  let no_value: Option<i32> = None;
  ```
  
##### Result enum
- The Result enum is used to handle error-prone situations. It is defined as follows:
``` rust
enum Result<T, E> {
  Ok(T),
  Err(E),
  }
```
- It represents either a successful value (Ok(T)) or an error (Err(E)).
This is useful when a computation might return a value or might fail to produce 
a value, along with an error message.
- eg
  ``` rust
  // A Result that can either contain an integer or an error message
  let success: Result<i32, &str> = Ok(42);
  let failure: Result<i32, &str> = Err("Something went wrong");
  ```
  - Common methods associated with Result include 
  unwrap(), is_ok(), is_err(), map(), and_then(), etc.
## Unwrap()
- The unwrap() method is a convenient way to handle the Result enum.
- It returns the value inside the Ok(T) variant if the result is Ok,
- otherwise, it panics and returns the error message inside the Err(E) variant.
- eg
  ``` rust
  let success: Result<i32, &str> = Ok(42);
  let failure: Result<i32, &str> = Err("Something went wrong");
  println!("{}", success.unwrap()); // 42
  println!("{}", failure.unwrap()); // panics with "Something went wrong"
  ```
- The unwrap() method is useful when you are confident that the result will be Ok.
- However, it is not recommended to use unwrap() in production code, as it can lead to panics.
- Instead, it is better to use match or the ? operator to handle the Result enum.
- The ? operator can be used to propagate errors from a function that returns a Result.
- It returns the value inside the Ok(T) variant if the result is Ok,
- otherwise, it returns the error message inside the Err(E) variant.
- eg
  ``` rust
  fn divide(x: f64, y: f64) -> Result<f64, &str> {
  if y == 0.0 {
  Err("Cannot divide by zero")
  } else {
  Ok(x / y)
  }
  }
  fn main() {
  let result = divide(10.0, 2.0)?;
  println!("{}", result); // 5.0
  let result = divide(10.0, 0.0)?;
  println!("{}", result); // panics with "Cannot divide by zero"
  }
  ```