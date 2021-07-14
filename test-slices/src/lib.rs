#[cfg(test)]
mod tests {
    #[test]
    fn array_slice() {
	let a = [1, 2, 3, 4, 5];

	let slice = &a[1..3];

        assert_eq!(slice, &[2, 3]);
    }

    #[test]
    fn vec_slice() {
	let a = vec![1, 2, 3, 4, 5];

	let slice = &a[1..3];

        assert_eq!(slice, &[2, 3]);	
    }

    #[test]
    fn string_slice() {
	let s = String::from("hello world");

	let hello = &s[0..5];
	let world = &s[6..11];

	assert_eq!(hello, "hello");
	assert_eq!(world, "world");

	//----------------------------------------------------------------
	// E0277
	// 31 |     let h = s[0..5];
	//    |         ^ doesn't have a size known at compile-time
	//    |
	//    = help: the trait `Sized` is not implemented for `str`
	//    = note: all local variables must have a statically known size
	//    = help: unsized locals are gated as an unstable feature
	//----------------------------------------------------------------
	// let h = s[0..5];
	// let w = s[6..11];
	// assert_eq!(h, *"hello");
	// assert_eq!(w, *"world");
    }
}
