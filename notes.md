
# Table of Contents

1.  [rustup](#org634267f)
2.  [cargo](#orgff42412)
3.  [cargo test](#org136a9d9)
4.  [rustc](#orgc228f73)
5.  [THOUGHTS](#orgb50cd12)
    1.  [Don't abuse variable shadowing.](#org13b222e)
    2.  [Don't use the "return value" of an operation.](#orge6aa7f4)
    3.  [A good example of the use of enum type (union type) to enfoce exception handling](#org937efdc)
6.  [Print type of a variable](#org3aa1e37)
7.  [&[String], &Vec<String> and [String; 5]](#orgd3df6be)
8.  [Reasons to adopt Rust in our projects](#org665545c)
9.  [Cons compared to Python](#org58c5f6b)
10. [Cons compared to Swift](#orgd60a56a)
11. [Cons compared to C++](#orgcb4fafd)
12. [Cons compared to Java 8](#orge20f211)
13. [Cons compared to Go](#orga2bb24f)
14. [Cons compared to Common Lisp](#org4d0ae8a)
15. [Compare with Oz](#org68d5822)
    1.  [pros](#org6fe0572)
    2.  [cons](#org60b63ad)
16. [Compared with Swift](#orga6e1491)
17. [It is a design error to use an empty tuple to denote the Void type](#org420be47)
18. [ANSI colors](#orgf0f5fcf)
19. [idioms](#orgeb5f7c4)
    1.  [open file and read to string](#org823fa10)
    2.  [return Ok(()) to indicate that the call is for its side effects only](#orga9d5bd8)
    3.  [how to specify the Fn trait bounds](#org444cfb5)
20. [Cautions](#org17082bd)
    1.  [std::process::exit()](#org3a9565a)
    2.  [str::to\_lowercase is not 100% accurate](#org754d69b)



<a id="org634267f"></a>

# rustup

<p class="verse">
rustup update<br />
rustup doc &#x2013;book<br />
</p>


<a id="orgff42412"></a>

# cargo

<p class="verse">
cargo run<br />
cargo run -p adder<br />
RUST\_BACKTRACE=1 cargo run &#x2013;verbose<br />
RUST\_BACKTRACE=full cargo run<br />
<br />
cargo build &#x2013;verbose<br />
<br />
cargo update<br />
<br />
cargo doc          # calls rustdoc<br />
cargo doc &#x2013;open<br />
<br />
cargo login [token]<br />
cargo publish<br />
cargo yank &#x2013;vers 1.0.1<br />
cargo yank &#x2013;vers 1.0.1 &#x2013;undo<br />
<br />
cargo install<br />
<br />
cargo &#x2013;list<br />
</p>


<a id="org136a9d9"></a>

# cargo test

<p class="verse">
cargo test &#x2013; &#x2013;test-threads=1<br />
cargo test &#x2013; &#x2013;show-output<br />
cargo test <test\_function\_name><br />
cargo test <sub\_string\_of\_test\_names\_including\_module\_names\_plus\_function\_names><br />
cargo test &#x2013; &#x2013;ignored<br />
cargo test &#x2013;test <integration\_test's crate\_name or function\_name><br />
cargo test -p <crate in a workspace><br />
</p>

In the **<proj\_root>/tests/** directory, Cargo will compile each of the files as
an individual crate.

Files in subdirectories of the tests directory don’t get compiled as separate
crates or have sections in the test output.

Common test files must be put in subdirectories, each directory represents a
module.  For example, to use "common" module, put the code for common module
in **<proj\_root>/tests/common/mod.rs**.  If we rename mod.rs to other name,
Cargo won't be able to find the module definition, because there is no
**<proj\_root>/tests/common.rs** as a guide.


<a id="orgc228f73"></a>

# rustc

<p class="verse">
rustc &#x2013;explain E0308<br />
</p>


<a id="orgb50cd12"></a>

# THOUGHTS


<a id="org13b222e"></a>

## Don't abuse variable shadowing.

> This feature is often used in situations in which you want to convert a value
> from one type to another type.  Shadowing lets us reuse the 'guess' variable
> name rahter than forcing us to create two unique variables, such as
> 'guess\_str' and 'guess' for example.

However, it makes people hard to find its definition.  Let's see if tools can
help find the accurate definition in one go.


<a id="orge6aa7f4"></a>

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


<a id="org937efdc"></a>

## A good example of the use of enum type (union type) to enfoce exception handling

<p class="verse">
&#xa0;&#xa0;&#xa0;Compiling guessing-game v0.1.0 (/Users/rduan/mygitlab/study-rust/guessing-game)<br />
error[E0308]: mismatched types<br />
&#xa0;&#xa0;&#x2013;> src/main.rs:20:22<br />
&#xa0;&#xa0;&#xa0;|<br />
20 |     let guess: u32 = guess.trim().parse();//.expect("Please type a number!");<br />
&#xa0;&#xa0;&#xa0;|                &#x2014;   ^^^^^^^^^^^^^^^^^^^^ expected \`u32\`, found enum \`Result\`<br />
&#xa0;&#xa0;&#xa0;|                |<br />
&#xa0;&#xa0;&#xa0;|                expected due to this<br />
&#xa0;&#xa0;&#xa0;|<br />
&#xa0;&#xa0;&#xa0;= note: expected type \`u32\`<br />
&#xa0;&#xa0;&#xa0;&#xa0;&#xa0;&#xa0;&#xa0;&#xa0;&#xa0;&#xa0;&#xa0;&#xa0;&#xa0;&#xa0;found enum \`Result<\_, \_>\`<br />
<br />
error: aborting due to previous error<br />
<br />
For more information about this error, try \`rustc &#x2013;explain E0308\`.<br />
error: could not compile \`guessing-game\`<br />
<br />
To learn more, run the command again with &#x2013;verbose.<br />
</p>


<a id="org3aa1e37"></a>

# Print type of a variable

    fn print_type_of<T>(_: &T) {
        println!("{}", std::any::type_name::<T>())
    }


<a id="orgd3df6be"></a>

# &[String], &Vec<String> and [String; 5]

[String; 5] is a primitive array type.
&[String] is a slice, it can be a slice of a vector or an array, etc.
&Vec<String> is a reference of Vec<String>.


<a id="org665545c"></a>

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


<a id="org58c5f6b"></a>

# Cons compared to Python

-   no REPL
-   harder to learn
-   burden to manipulate the object memory ownership
-   less mature in the ecosystem
-   a little less coding speed


<a id="orgd60a56a"></a>

# Cons compared to Swift

-   no REPL
-   syntax being more elaborate


<a id="orgcb4fafd"></a>

# Cons compared to C++

-   less mature in the ecosystem


<a id="orge20f211"></a>

# Cons compared to Java 8

-   less mature in the ecosystem


<a id="orga2bb24f"></a>

# Cons compared to Go

-   less mature in the ecosystem
-   harder to learn


<a id="org4d0ae8a"></a>

# Cons compared to Common Lisp

-   no REPL
-   no meta programming control in the compile time and load time
-   not "data as code" and "code as data"


<a id="org68d5822"></a>

# Compare with Oz


<a id="org6fe0572"></a>

## pros

-   much better string type
-   better runtime performance
-   smaller memory footprint, e.g. functions don't capture context variables


<a id="org60b63ad"></a>

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


<a id="orga6e1491"></a>

# Compared with Swift

<https://dev.to/rhymu8354/swift-vs-rust-an-overview-of-swift-from-a-rusty-perspective-18c7>


<a id="org420be47"></a>

# It is a design error to use an empty tuple to denote the Void type

I like the Oz's design: an operation does not return anything.


<a id="orgf0f5fcf"></a>

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


<a id="orgeb5f7c4"></a>

# idioms


<a id="org823fa10"></a>

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


<a id="orga9d5bd8"></a>

## return Ok(()) to indicate that the call is for its side effects only


<a id="org444cfb5"></a>

## how to specify the Fn trait bounds

Most of the time when specifying one of the Fn trait bounds, you can start
with Fn and the compiler will tell you if you need FnMut or FnOnce based on
what happens in the closure body.


<a id="org17082bd"></a>

# Cautions


<a id="org3a9565a"></a>

## std::process::exit()

<https://doc.rust-lang.org/std/process/fn.exit.html>
[Is Rust Cleaning Up After Exit](https://users.rust-lang.org/t/is-rust-cleaning-up-after-exit/9613)


<a id="org754d69b"></a>

## str::to\_lowercase is not 100% accurate

From the Rust book:

> While to\_lowercase will handle basic Unicode, it won't be 100% accurate.  If we
> were writing a real application, we'd want to do a bit more work here.

