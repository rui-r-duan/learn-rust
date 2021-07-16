
# Table of Contents

1.  [rustup](#org5376dc9)
2.  [cargo](#orgc1bfb63)
3.  [cargo test](#orgd75c885)
4.  [rustc](#org2f8d433)
5.  [THOUGHTS](#orge991ca1)
    1.  [Don't abuse variable shadowing.](#orge76d15e)
    2.  [Don't use the "return value" of an operation.](#orga3f7999)
    3.  [A good example of the use of enum type (union type) to enfoce exception handling](#org164d944)
6.  [Print type of a variable](#orgcf4bd46)
7.  [&[String], &Vec<String> and [String; 5]](#orgdf10216)
8.  [Reasons to adopt Rust in our projects](#org03b68a1)
9.  [Cons compared to Python](#orge7f9454)
10. [Cons compared to Swift](#org991468d)
11. [Cons compared to C++](#orged43206)
12. [Cons compared to Java 8](#orgefb22cf)
13. [Cons compared to Go](#org9bcd8dd)
14. [Cons compared to Common Lisp](#org34b618a)
15. [Compare with Oz](#org49f8f0a)
    1.  [pros](#org8ac69c3)
    2.  [cons](#org06b65ee)
16. [Compared with Swift](#org30dfbf2)
17. [It is a design error to use an empty tuple to denote the Void type](#org3e956bb)
18. [ANSI colors](#org4c97601)
19. [Idioms](#org08bca3e)
    1.  [Open file and read to string](#orgfcc0031)
    2.  [Return Ok(()) to indicate that the call is for its side effects only](#org54f1767)
    3.  [Return \`Result<(), Box<dyn Error>>\` for general run() method](#orga770c45)
    4.  [How to specify the Fn trait bounds](#orgbe9344e)
    5.  [Compile time polymorphism (static dispatch): bounded parametric polymorphism](#org7bb0258)
    6.  [Run time polymorphism (duck typing, dynamic dispatch): trait object](#org37cc1e5)
        1.  [Pros compared to duck typing in dynamically typed languages](#org0b6d7f7)
        2.  [Pros compared to closures as objects](#orgb7aa2be)
    7.  [Use enum to let Vec<T> hold limited different types](#org7f66c17)
    8.  [Option<T>](#org3d4f865)
20. [Cautions](#orgfed628c)
    1.  [std::process::exit()](#org31cd18a)
    2.  [str::to\_lowercase is not 100% accurate](#org3d832fe)



<a id="org5376dc9"></a>

# rustup

    rustup update
    rustup doc --book


<a id="orgc1bfb63"></a>

# cargo

    cargo run
    cargo run -p adder
    RUST_BACKTRACE=1 cargo run --verbose
    RUST_BACKTRACE=full cargo run
    
    cargo build --verbose
    
    cargo update
    
    cargo doc          # calls rustdoc
    cargo doc --open
    
    cargo login [token]
    cargo publish
    cargo yank --vers 1.0.1
    cargo yank --vers 1.0.1 --undo
    
    cargo install
    
    cargo --list


<a id="orgd75c885"></a>

# cargo test

    cargo test -- --test-threads=1
    cargo test -- --show-output
    cargo test <test_function_name>
    cargo test <sub_string_of_test_names_including_module_names_plus_function_names>
    cargo test -- --ignored
    cargo test --test <integration_test's crate_name or function_name>
    cargo test -p <crate in a workspace>

In the **<proj\_root>/tests/** directory, Cargo will compile each of the files as
an individual crate.

Files in subdirectories of the tests directory don’t get compiled as separate
crates or have sections in the test output.

Common test files must be put in subdirectories, each directory represents a
module.  For example, to use "common" module, put the code for common module
in **<proj\_root>/tests/common/mod.rs**.  If we rename mod.rs to other name,
Cargo won't be able to find the module definition, because there is no
**<proj\_root>/tests/common.rs** as a guide.


<a id="org2f8d433"></a>

# rustc

    rustc --explain E0308


<a id="orge991ca1"></a>

# THOUGHTS


<a id="orge76d15e"></a>

## Don't abuse variable shadowing.

> This feature is often used in situations in which you want to convert a value
> from one type to another type.  Shadowing lets us reuse the 'guess' variable
> name rahter than forcing us to create two unique variables, such as
> 'guess\_str' and 'guess' for example.

However, it makes people hard to find its definition.  Let's see if tools can
help find the accurate definition in one go.


<a id="orga3f7999"></a>

## Don't use the "return value" of an operation.

    let mut y = 5;
    let x = (y = 6);		// x has the value '()', not '6'

This feature is to make sure that every expression to have a value, including
the assignment operation/expression.

The fundamental design fault is that they use an empty tuple to denote a void
type.

Rust allows you to propogate a meaningless return type of an statement like
expression.  If this kind of error is not caught and is propogated to other
places, it will be difficult to find the root cause, and this issue may cause
runtime errors or type checking errors.

The same design fault lives in JavaScript and PHP where the meaningless
"undefined" is spreaded over the whole program.


<a id="org164d944"></a>

## A good example of the use of enum type (union type) to enfoce exception handling

       Compiling guessing-game v0.1.0 (/Users/rduan/mygitlab/study-rust/guessing-game)
    error[E0308]: mismatched types
      --> src/main.rs:20:22
       |
    20 |     let guess: u32 = guess.trim().parse();//.expect("Please type a number!");
       |                ---   ^^^^^^^^^^^^^^^^^^^^ expected `u32`, found enum `Result`
       |                |
       |                expected due to this
       |
       = note: expected type `u32`
    	      found enum `Result<_, _>`
    
    error: aborting due to previous error
    
    For more information about this error, try `rustc --explain E0308`.
    error: could not compile `guessing-game`
    
    To learn more, run the command again with --verbose.


<a id="orgcf4bd46"></a>

# Print type of a variable

    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }


<a id="orgdf10216"></a>

# &[String], &Vec<String> and [String; 5]

[String; 5] is a primitive array type.

&[String] is a slice, it can be a slice of a vector or an array, etc.

&Vec<String> is a reference of Vec<String>.


<a id="org03b68a1"></a>

# Reasons to adopt Rust in our projects

-   advanced tool to aid us to design and code
    -   type safety
        -   no the billion-dollar bug (null bug)
        -   safer error handling than Go and other nullable language, compiler can
            aid us and protect us
    -   easy concurrency and reasoning (human understanding) by declarative
        computing
        -   immutable variable by default
        -   support closure which facilitates functional programming paradigm
    -   modern built-in types and types from the standard library (using old
        languages, we have to tweak the old types for good practices nowadays)
-   prototyping and enough optimization in one go, compared to quick
    prototyping and hard and long optimization in Python
-   save cloud cost by
    -   extremely high runtime speed (can serve more requests per second)
        comparable to C/C++
    -   small memory footprints
    -   small container images
-   modern tooling
    -   very helpful compiler
    -   specialized build system included (i.e. cargo) vs. C++'s GNU Make
    -   documentation tools (e.g. rustup doc)
    -   good test framework to support TDD
    -   modern package and module system (good trade-off between conventions and
        flexibility)
-   expressive and concise
    -   less boilerplate code than Java and C++
    -   more expressive and elegant than Go
        -   support Generic
            -   consider a use case where we need to sort entries of a user defined
                type, Go's implementation is tedious
            -   In Go, people tend to abuse interface{} to hold unknown type of data,
                which bypasses the type safety checking
        -   better trade-off than Go
            -   Go hard-codes hash table into the language, whereas Rust uses library
-   modern package and module management
-   good community
    -   strong language developers
    -   friendly and supportive
    -   backing from large companies (Mozilla, Microsoft, Amazon, Google, etc.)
-   better FFI than Go


<a id="orge7f9454"></a>

# Cons compared to Python

-   no REPL
-   harder to learn
-   burden to manipulate the object memory ownership
-   less mature in the ecosystem
-   a little less coding speed


<a id="org991468d"></a>

# Cons compared to Swift

-   no REPL
-   syntax being more elaborate


<a id="orged43206"></a>

# Cons compared to C++

-   less mature in the ecosystem


<a id="orgefb22cf"></a>

# Cons compared to Java 8

-   less mature in the ecosystem


<a id="org9bcd8dd"></a>

# Cons compared to Go

-   less mature in the ecosystem
-   harder to learn


<a id="org34b618a"></a>

# Cons compared to Common Lisp

-   no REPL
-   not "data as code" and "code as data"
-   no multiple dynamic dispatch


<a id="org49f8f0a"></a>

# Compare with Oz


<a id="org8ac69c3"></a>

## pros

-   much better string type
-   better runtime performance
-   smaller memory footprint, e.g. functions don't capture context variables


<a id="org06b65ee"></a>

## cons

-   no tail call optimization which leads to a crippled support for recursion,
    which leads to less descriptive programming paradigm.
-   tedious to express recursive types (structures), such as linked list,
    trees, and graphs, unless using raw pointers.  This is due to lack of GC.
-   no difference list
-   conceptually more complicated types for pattern matching.  In Oz, Record
    type suffices.
-   quirks due to the expression-based design
    I like Oz's design better: procedure and functions are different types.
    e.g.
    -   operations return an empty tuple
    -   placing a semicolon turns an expression to a statement
-   functions are not closures, so the functional programming is limited
-   due to lack of GC, closures are complicated: it either borrows or moves
    captured values


<a id="org30dfbf2"></a>

# Compared with Swift

<https://dev.to/rhymu8354/swift-vs-rust-an-overview-of-swift-from-a-rusty-perspective-18c7>


<a id="org3e956bb"></a>

# It is a design error to use an empty tuple to denote the Void type

I like the Oz's design: an operation does not return anything.


<a id="org4c97601"></a>

# ANSI colors

    const ANSI_BLACK: &str = "\x1b[0;30m";
    const ANSI_RED: &str = "\x1b[0;31m";
    const ANSI_GREEN: &str = "\x1b[0;32m";
    const ANSI_YELLOW: &str = "\x1b[0;33m";
    const ANSI_BLUE: &str = "\x1b[0;34m";
    const ANSI_MAGENTA: &str = "\x1b[0;35m";
    const ANSI_CYAN: &str = "\x1b[0;36m";
    const ANSI_WHITE: &str = "\x1b[0;37m";
    const ANSI_RESET: &str = "\x1b[0m";


<a id="org08bca3e"></a>

# Idioms


<a id="orgfcc0031"></a>

## Open file and read to string

The following four versions are equivalent.

    // Version 1
    use std::fs;
    use std::io;
    
    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    // Version 2
    use std::fs::File;
    use std:io;
    use std::io::Read;
    
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }

    // Version 3
    use std::fs::File;
    use std::io;
    use std::io::Read;
    
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    // Version 4
    use std::fs::File;
    use std::io;
    use std::io::Read;
    
    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");
    
        let mut f = match f {
    	Ok(file) => file,
    	Err(e) => return Err(e),
        };
    
        let mut s = String::new();
    
        match f.read_to_string(&mut s) {
    	Ok(_) => Ok(s),
    	Err(e) => Err(e),
        }
    }


<a id="org54f1767"></a>

## Return Ok(()) to indicate that the call is for its side effects only


<a id="orga770c45"></a>

## Return \`Result<(), Box<dyn Error>>\` for general run() method


<a id="orgbe9344e"></a>

## How to specify the Fn trait bounds

Most of the time when specifying one of the Fn trait bounds, you can start
with Fn and the compiler will tell you if you need FnMut or FnOnce based on
what happens in the closure body.


<a id="org7bb0258"></a>

## Compile time polymorphism (static dispatch): bounded parametric polymorphism

**Generics** and **Trait bounds** is preferable for homogeneous collections,
because the definitions will be *monomorphized* at compile time to use the
concret types.

    pub struct Screen<T: Draw> {
        // The vector can hold only one type of objects, such as Button or TextField.
        pub components: Vec<T>,
    }
    
    impl<T> Screen<T>
    where
        T: Draw,
    {
        pub fn run(&self) {
    	for component in self.components.iter() {
    	    component.draw();
    	}
        }
    }


<a id="org37cc1e5"></a>

## Run time polymorphism (duck typing, dynamic dispatch): trait object

It is like a C++ object which binds behaviors with data.  In C++, the
dynamic behaviors are implemented by a virtual table in an object.  In a
Rust's trait object, such kind of virtual table is also embedded.

    pub trait Draw {
        fn draw(&self);
    }
    
    pub struct Screen {
        // The vector can hold any objects that implements the `Draw` trait,
        // such as Box<Button> and Box<TextField>.
        pub components: Vec<Box<dyn Draw>>,
    }


<a id="org0b6d7f7"></a>

### Pros compared to duck typing in dynamically typed languages

> The advantage of using trait objects and Rust’s type system to write code
> similar to code using duck typing is that we never have to check whether a
> value implements a particular method at runtime or worry about getting errors
> if a value doesn’t implement a method but we call it anyway. Rust won’t compile
> our code if the values don’t implement the traits that the trait objects need.


<a id="orgb7aa2be"></a>

### Pros compared to closures as objects

A closure can binds data and behaviors together, thus it can be used as an
\`object\`.  However, it is not easy to tell what type of object it is, for
example, whether it implements \`draw()\` method or not.

Duck typing in dynamically typed languages is implemented in a similar way
as closures.


<a id="org7f66c17"></a>

## Use enum to let Vec<T> hold limited different types

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


<a id="org3d4f865"></a>

## Option<T>

    option.expect()
    option.take()
    option.as_ref()
    option.is_some()
    option.is_none()
    option.unwrap()


<a id="orgfed628c"></a>

# Cautions


<a id="org31cd18a"></a>

## std::process::exit()

<https://doc.rust-lang.org/std/process/fn.exit.html>

[Is Rust Cleaning Up After Exit](https://users.rust-lang.org/t/is-rust-cleaning-up-after-exit/9613)


<a id="org3d832fe"></a>

## str::to\_lowercase is not 100% accurate

From the Rust book:

> While to\_lowercase will handle basic Unicode, it won't be 100% accurate.  If we
> were writing a real application, we'd want to do a bit more work here.

