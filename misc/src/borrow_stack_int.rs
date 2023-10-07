fn main() {
    //----------------------------------------------------------------
    // TWO MUT REFERENCES
    //----------------------------------------------------------------
    // let mut data = 10;
    // let ref1 = &mut data;
    // let ref2 = &mut *ref1;

    // // ORDER SWAPPED!
    // *ref1 += 1;
    // *ref2 += 2;

    // println!("{}", data);

    //----------------------------------------------------------------
    // let mut data = 10;
    // let ref1 = &mut data;
    // let ptr2 = ref1 as *mut _;

    // // ORDER SWAPPED!
    // *ref1 += 1;
    // unsafe {
    // 	*ptr2 += 2;
    // }

    // println!("{}", data);
    //----------------------------------------------------------------

    //----------------------------------------------------------------
    unsafe {
	let mut data = 10;
	let ref1 = &mut data;
	let ptr2 = ref1 as *mut _;
	let ref3 = &mut *ptr2;
	let ptr4 = ref3 as *mut _;

	// Access the first raw pointer first
	*ptr2 += 2;

	// Then access things in "borrow stack" order
	*ptr4 += 4;
	*ref3 += 3;
	*ptr2 += 2;
	*ref1 += 1;

	println!("{}", data);
    }
    //----------------------------------------------------------------
}
