
# Table of Contents

1.  [rustup](#org3cdac8f)
2.  [cargo](#org338958f)
3.  [cargo test](#org2b24378)
4.  [rustc](#org85d337e)
5.  [THOUGHTS](#orgd41b09b)
    1.  [Don't abuse variable shadowing.](#org9ed05b3)
    2.  [Don't use the "return value" of an operation.](#org7d860a7)
    3.  [A good example of the use of enum type (union type) to enfoce exception handling](#orga98a0cf)
6.  [Print type of a variable](#orgc59e253)
7.  [&[String], &Vec<String> and [String; 5]](#org5e81567)
8.  [Reasons to adopt Rust in our projects](#org03c3b7d)
9.  [Cons compared to Python](#org7c1dc53)
10. [Cons compared to Swift](#org8ecea3e)
11. [Cons compared to C++](#orgfdb0ede)
12. [Cons compared to Java 8](#orgf21b677)
13. [Cons compared to Go](#org88c210c)
14. [Cons compared to Common Lisp](#org62a812b)
15. [Compare with Oz](#orgd31622a)
    1.  [pros](#org8184eca)
    2.  [cons](#org1214f28)
16. [Compared with Swift](#org60ac028)
17. [It is a design error to use an empty tuple to denote the Void type](#orgc81524b)
18. [ANSI colors](#orgc02091c)
19. [idioms](#org0eaf0ca)
    1.  [open file and read to string](#orga7eb5e6)
    2.  [return Ok(()) to indicate that the call is for its side effects only](#orgf619975)
    3.  [how to specify the Fn trait bounds](#org20b8451)
20. [Cautions](#orge51f76b)
    1.  [std::process::exit()](#org9c7924d)
    2.  [str::to\_lowercase is not 100% accurate](#org11ce662)



<a id="org3cdac8f"></a>

# rustup

    rustup update
    rustup doc --book


<a id="org338958f"></a>

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


<a id="org2b24378"></a>

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


<a id="org85d337e"></a>

# rustc

    rustc --explain E0308


<a id="orgd41b09b"></a>

# THOUGHTS


<a id="org9ed05b3"></a>

## Don't abuse variable shadowing.

> This feature is often used in situations in which you want to convert a value
> from one type to another type.  Shadowing lets us reuse the 'guess' variable
> name rahter than forcing us to create two unique variables, such as
> 'guess\_str' and 'guess' for example.

However, it makes people hard to find its definition.  Let's see if tools can
help find the accurate definition in one go.


<a id="org7d860a7"></a>

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


<a id="orga98a0cf"></a>

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


<a id="orgc59e253"></a>

# Print type of a variable

    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }


<a id="org5e81567"></a>

# &[String], &Vec<String> and [String; 5]

[String; 5] is a primitive array type.
&[String] is a slice, it can be a slice of a vector or an array, etc.
&Vec<String> is a reference of Vec<String>.


<a id="org03c3b7d"></a>

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
    -   backing by large companies (Mozilla, Microsoft, Amazon, Google, etc.)
-   better FFI than Go


<a id="org7c1dc53"></a>

# Cons compared to Python

-   no REPL
-   harder to learn
-   burden to manipulate the object memory ownership
-   less mature in the ecosystem
-   a little less coding speed


<a id="org8ecea3e"></a>

# Cons compared to Swift

-   no REPL
-   syntax being more elaborate


<a id="orgfdb0ede"></a>

# Cons compared to C++

-   less mature in the ecosystem


<a id="orgf21b677"></a>

# Cons compared to Java 8

-   less mature in the ecosystem


<a id="org88c210c"></a>

# Cons compared to Go

-   less mature in the ecosystem
-   harder to learn


<a id="org62a812b"></a>

# Cons compared to Common Lisp

-   no REPL
-   no meta programming control in the compile time and load time
-   not "data as code" and "code as data"


<a id="orgd31622a"></a>

# Compare with Oz


<a id="org8184eca"></a>

## pros

-   much better string type
-   better runtime performance
-   smaller memory footprint, e.g. functions don't capture context variables


<a id="org1214f28"></a>

## cons

-   no tail call optimization which leads to a crippled support for recursion,
    which leads to less descriptive programming paradigm.
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


<a id="org60ac028"></a>

# Compared with Swift

<https://dev.to/rhymu8354/swift-vs-rust-an-overview-of-swift-from-a-rusty-perspective-18c7>


<a id="orgc81524b"></a>

# It is a design error to use an empty tuple to denote the Void type

I like the Oz's design: an operation does not return anything.


<a id="orgc02091c"></a>

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


<a id="org0eaf0ca"></a>

# idioms


<a id="orga7eb5e6"></a>

## open file and read to string

The following four versions are equivalent.

    use std::fs;
    use std::io;
    
    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    use std::fs::File;
    use std:io;
    use std::io::Read;
    
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }

    use std::fs::File;
    use std::io;
    use std::io::Read;
    
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

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


<a id="orgf619975"></a>

## return Ok(()) to indicate that the call is for its side effects only


<a id="org20b8451"></a>

## how to specify the Fn trait bounds

Most of the time when specifying one of the Fn trait bounds, you can start
with Fn and the compiler will tell you if you need FnMut or FnOnce based on
what happens in the closure body.


<a id="orge51f76b"></a>

# Cautions


<a id="org9c7924d"></a>

## std::process::exit()

<https://doc.rust-lang.org/std/process/fn.exit.html>
[Is Rust Cleaning Up After Exit](https://users.rust-lang.org/t/is-rust-cleaning-up-after-exit/9613)


<a id="org11ce662"></a>

## str::to\_lowercase is not 100% accurate

From the Rust book:

> While to\_lowercase will handle basic Unicode, it won't be 100% accurate.  If we
> were writing a real application, we'd want to do a bit more work here.

