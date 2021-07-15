fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let f32 = 5;
    let mut number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0]; // does largest own the object "34"?
    print_type_of(&largest);
    print_type_of(&number_list[0]);

    for number in &number_list {
        if number > &largest {
            largest = *number;
        }
    }

    println!("The largest number is {}", largest);
    assert_eq!(largest, 100);

    println!("{}", number_list[0]);
    number_list[0] = 200;
    println!("{}", number_list[0]);

    println!("{}", number_list[3]); // the same object as largest
    number_list[3] = 5000;
    println!("{}", number_list[3]);
}
