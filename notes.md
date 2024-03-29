
# Table of Contents

1.  [rustup](#org1d64fa0)
2.  [cargo](#orga93b2d3)
3.  [cargo test](#org909a67b)
4.  [rustc](#org7fbfd7a)
5.  [THOUGHTS](#org5ae4a24)
    1.  [Don't abuse variable shadowing.](#org5002c93)
    2.  [Don't use the "return value" of an operation.](#org3801708)
    3.  [A good example of the use of enum type (union type) to enfoce exception handling](#orga882a37)
6.  [Print type of a variable](#org1cdebee)
7.  [&[String], &Vec<String> and [String; 5]](#orga98a103)
8.  [Reasons to adopt Rust in our projects](#orgc032d51)
9.  [Cons compared to Python](#org533665b)
10. [Cons compared to Swift](#orge38d211)
11. [Cons compared to C++](#org9a30a33)
12. [Cons compared to Java 8](#org620b1ed)
13. [Cons compared to Go](#org1c3af2c)
14. [Cons compared to Common Lisp](#orgc30eb19)
15. [Compare with Oz](#org4d070fb)
    1.  [pros](#orga180e6d)
    2.  [cons](#org1a30df9)
16. [Compared with Swift](#org073cb23)
17. [It is a design error to use an empty tuple to denote the Void type](#orgfeecd2f)
18. [ANSI colors](#orgf666e97)
19. [Idioms](#org21b2c5c)
    1.  [Open file and read to string](#orge9b1505)
    2.  [Return Ok(()) to indicate that the call is for its side effects only](#org0ee9f11)
    3.  [Return \`Result<(), Box<dyn Error>>\` for general run() method](#orgb71ecdc)
    4.  [How to specify the Fn trait bounds](#org8072518)
    5.  [Compile time polymorphism (static dispatch): bounded parametric polymorphism](#orgb637afc)
    6.  [Run time polymorphism (duck typing, dynamic dispatch): trait object](#org1165c00)
        1.  [Pros compared to duck typing in dynamically typed languages](#org2abf15c)
        2.  [Pros compared to closures as objects](#orgf046ef2)
    7.  [Use enum to let Vec<T> hold limited different types](#org53d899e)
    8.  [Option<T>](#org6164213)
20. [Cautions](#org638cf48)
    1.  [std::process::exit()](#org084dae6)
    2.  [str::to\_lowercase is not 100% accurate](#org67caff0)



<a id="org1d64fa0"></a>

# rustup

    rustup update
    rustup doc --book


<a id="orga93b2d3"></a>

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


<a id="org909a67b"></a>

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


<a id="org7fbfd7a"></a>

# rustc

    rustc --explain E0308


<a id="org5ae4a24"></a>

# THOUGHTS


<a id="org5002c93"></a>

## Don't abuse variable shadowing.

> This feature is often used in situations in which you want to convert a value
> from one type to another type.  Shadowing lets us reuse the 'guess' variable
> name rahter than forcing us to create two unique variables, such as
> 'guess\_str' and 'guess' for example.

However, it makes people hard to find its definition.  Let's see if tools can
help find the accurate definition in one go.


<a id="org3801708"></a>

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


<a id="orga882a37"></a>

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


<a id="org1cdebee"></a>

# Print type of a variable

    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }


<a id="orga98a103"></a>

# &[String], &Vec<String> and [String; 5]

[String; 5] is a primitive array type.

&[String] is a slice, it can be a slice of a vector or an array, etc.

&Vec<String> is a reference of Vec<String>.


<a id="orgc032d51"></a>

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


<a id="org533665b"></a>

# Cons compared to Python

-   no REPL
-   harder to learn
-   burden to manipulate the object memory ownership
-   less mature in the ecosystem
-   a little less coding speed


<a id="orge38d211"></a>

# Cons compared to Swift

-   no REPL
-   syntax being more elaborate


<a id="org9a30a33"></a>

# Cons compared to C++

-   less mature in the ecosystem


<a id="org620b1ed"></a>

# Cons compared to Java 8

-   less mature in the ecosystem


<a id="org1c3af2c"></a>

# Cons compared to Go

-   less mature in the ecosystem
-   harder to learn


<a id="orgc30eb19"></a>

# Cons compared to Common Lisp

-   no REPL
-   harder to use macros, so harder to implement "data as code" and "code as data"


<a id="org4d070fb"></a>

# Compare with Oz


<a id="orga180e6d"></a>

## pros

-   much better string type
-   better runtime performance
-   smaller memory footprint, e.g. functions don't capture context variables


<a id="org1a30df9"></a>

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


<a id="org073cb23"></a>

# Compared with Swift

<https://dev.to/rhymu8354/swift-vs-rust-an-overview-of-swift-from-a-rusty-perspective-18c7>


<a id="orgfeecd2f"></a>

# It is a design error to use an empty tuple to denote the Void type

I like the Oz's design: an operation does not return anything.


<a id="orgf666e97"></a>

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


<a id="org21b2c5c"></a>

# Idioms


<a id="orge9b1505"></a>

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


<a id="org0ee9f11"></a>

## Return Ok(()) to indicate that the call is for its side effects only


<a id="orgb71ecdc"></a>

## Return \`Result<(), Box<dyn Error>>\` for general run() method


<a id="org8072518"></a>

## How to specify the Fn trait bounds

Most of the time when specifying one of the Fn trait bounds, you can start
with Fn and the compiler will tell you if you need FnMut or FnOnce based on
what happens in the closure body.


<a id="orgb637afc"></a>

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


<a id="org1165c00"></a>

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


<a id="org2abf15c"></a>

### Pros compared to duck typing in dynamically typed languages

> The advantage of using trait objects and Rust’s type system to write code
> similar to code using duck typing is that we never have to check whether a
> value implements a particular method at runtime or worry about getting errors
> if a value doesn’t implement a method but we call it anyway. Rust won’t compile
> our code if the values don’t implement the traits that the trait objects need.


<a id="orgf046ef2"></a>

### Pros compared to closures as objects

A closure can binds data and behaviors together, thus it can be used as an
\`object\`.  However, it is not easy to tell what type of object it is, for
example, whether it implements \`draw()\` method or not.

Duck typing in dynamically typed languages is implemented in a similar way
as closures.


<a id="org53d899e"></a>

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


<a id="org6164213"></a>

## Option<T>

    option.expect()
    option.take()
    option.as_ref()
    option.is_some()
    option.is_none()
    option.unwrap()


<a id="org638cf48"></a>

# Cautions


<a id="org084dae6"></a>

## std::process::exit()

<https://doc.rust-lang.org/std/process/fn.exit.html>

[Is Rust Cleaning Up After Exit](https://users.rust-lang.org/t/is-rust-cleaning-up-after-exit/9613)


<a id="org67caff0"></a>

## str::to\_lowercase is not 100% accurate

From the Rust book:

> While to\_lowercase will handle basic Unicode, it won't be 100% accurate.  If we
> were writing a real application, we'd want to do a bit more work here.

