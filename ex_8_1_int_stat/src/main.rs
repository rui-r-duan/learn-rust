use std::convert::TryInto;
use std::collections::HashMap;

fn mean(a: &Vec<i32>) -> i32 {
    let len: i32 = a.len().try_into().unwrap();
    let mut sum = 0;
    for i in a.iter() {
        sum += i;
    }
    sum / len
}

fn median(a: &Vec<i32>) -> i32 {
    let len = a.len();
    let mid = len >> 1;
    a[mid]
}

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>());
// }

fn mode(a: &Vec<i32>) -> i32 {
    let mut count: HashMap<i32, i32> = HashMap::new();
    for i in a.iter() {
	let n = count.entry(*i).or_insert(0);
	*n += 1;
    }
    let (mut max_key, mut max_freq) = (-1, -1);
    // print_type_of(&max_key);
    // print_type_of(&max_freq);
    for (&k, &c) in &count {
	// println!("{} {}", k, c);
	// print_type_of(&k);
	// print_type_of(&c);
	if c > max_freq {
	    max_freq = c;
	    max_key = k;
	}
    }
    max_key
}

fn main() {
    let mut a = vec![5, 5, 5, 8, 20, 6, 9, 25, -3, -3, -3, -3, -17, 0, 29, 80];

    a.sort_unstable();

    println!("{:?}", a);
    println!("mean = {}", mean(&a));
    println!("median = {}", median(&a));
    println!("mode = {}", mode(&a));
}
