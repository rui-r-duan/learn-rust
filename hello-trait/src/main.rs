
//----------------------------------------------------------------
// VERSION 1
//----------------------------------------------------------------
// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

//----------------------------------------------------------------
// VERSION 2
//----------------------------------------------------------------
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest_index = 0;

    let mut i = 0;
    for item in list {
        if item > &list[largest_index] {
            largest_index = i;
        }
	i += 1;
    }

    &list[largest_index]
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
