fn main() {
    // unsafe {
    //     let mut data: Box<i32> = Box::new(10);
    //     let ptr1: *mut i32 = (&mut *data) as *mut i32;

    //     *data += 10;
    //     // error: Undefined Behavior: attempting a read access using
    //     // <1536> at alloc787[0x0], but that tag does not exist in the
    //     // borrow stack for this location
    //     *ptr1 += 1;

    //     // Should be 21
    //     println!("{}", data);
    // }

    unsafe {
        let mut data = Box::new(10);
        let ptr1 = (&mut *data) as *mut i32;

        *ptr1 += 1;
        *data += 10;

        // Should be 21
        println!("{}", data);
    }
}
