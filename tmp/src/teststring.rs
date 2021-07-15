fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let hello = "Здравствуйте";
    print_type_of(&hello);

    // error[E0277]: the size for values of type `str` cannot be known at compilation time
    print_type_of(&"Здравствуйте");

    let hello = hello.to_string();
    print_type_of(&hello);

    // let answer = &hello[0];
    // println!("{}", answer);
}
