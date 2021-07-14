use std::collections::HashMap;

pub struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    pub fn new(calculation: T) -> Cacher<T> {
	Cacher {
	    calculation,
	    value: HashMap::new(),
	}
    }

    pub fn value(&mut self, arg: u32) -> u32 {
	if self.value.contains_key(&arg) {
	    self.value.get(&arg).cloned().unwrap()
	}
	else {
	    // to call the function stored in `calculation`, surround the field
	    // access with parentheses
	    let v = (self.calculation)(arg);
	    self.value.insert(arg, v);
	    v
	}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
	let mut c = Cacher::new(|a| a);

	let _v1 = c.value(1);
	let v2 = c.value(2);

	assert_eq!(v2, 2);
    }

    #[test]
    fn test_modify_captured_var_outter() {
	let mut x = 4;

	// fn equal_to_x(z: i32) -> bool {
	// 	z == x
	// }
	let equal_to_x = |z| z == x;

	//----------------------------------------------------------------
	// error[E0506]: cannot assign to `x` because it is borrowed
	// 	--> src/main.rs:9:5
	// 	|
	// 7  |     let equal_to_x = |z| z == x;
	// |                      ---      - borrow occurs due to use in closure
	// 	|                      |
	// |                      borrow of `x` occurs here
	// 	8  |
	// 9  |     x = 6;
	// |     ^^^^^ assignment to borrowed `x` occurs here
	// 	...
	// 	13 |     assert!(equal_to_x(y));
	// |             ---------- borrow later used here
	// 	error: aborting due to previous error; 1 warning emitted
	// 	For more information about this error, try `rustc --explain E0506`.
	//----------------------------------------------------------------
	// x = 6;			// E0506

	let y = 4;

	assert!(equal_to_x(y));
    }

    #[test]
    fn test_modify_captured_var_inner() {
	let mut x = 4; // without mut, in the closure, the assignment is forbidden

	let mut add_five = |z| {
	    // borrow of `x` occurs in the closure
	    // borrow occurs due to use of `x` in closure
	    x = 5;
	    z + x
	};

	// print:
	// hello_closure::tests::test_modify_captured_var_inner::{{closure}}
	print_type_of(&add_five);

	assert_eq!(add_five(6), 11);
    }

    fn print_type_of<T>(_: &T) {
	println!("{}", std::any::type_name::<T>())
    }
}
