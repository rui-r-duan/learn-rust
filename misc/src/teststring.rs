fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let hello = "Здравствуйте";
    print_type_of(&hello); // &str

    // error[E0277]: the size for values of type `str` cannot be known at compilation time
    // print_type_of("Здравствуйте");
    print_type_of(&"Здравствуйте"); // &str

    let hello = hello.to_string();
    print_type_of(&hello); // alloc::string::String

    let answer = hello.get(0..1);
    println!("{answer:?}"); // None

    let x: String = "3".to_string();
    let answer = x.get(0..1);
    println!("{answer:?}"); // Some("3")
}
