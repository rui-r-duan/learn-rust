fn main() {
    fn opaque_read(val: &i32) {
        println!("{}", val);
    }

    //----------------------------------------------------------------
    // let mut data = 10;
    // let mref1 = &mut data;
    // let sref2 = &mref1;
    // let sref3 = sref2;
    // let sref4 = &*sref2;

    // // Random hash of shared reference reads
    // opaque_read(sref3);
    // opaque_read(sref2);
    // opaque_read(sref4);
    // opaque_read(sref2);
    // opaque_read(sref3);

    // *mref1 += 1;

    // opaque_read(&data);
    //----------------------------------------------------------------

    //----------------------------------------------------------------
    // unsafe {
    //     let mut data = 10;
    //     let mref1 = &mut data;
    //     let ptr2 = mref1 as *mut i32;
    //     let sref3 = &*mref1;
    //     let ptr4 = sref3 as *const i32 as *mut i32;

    //     *ptr4 += 4;
    //     opaque_read(sref3);
    //     *ptr2 += 2;
    //     *mref1 += 1;

    //     opaque_read(&data);
    // }
    //----------------------------------------------------------------

    //----------------------------------------------------------------
    // unsafe {
    //     let mut data = 10;
    //     let mref1 = &mut data;
    //     let ptr2 = mref1 as *mut i32;
    //     let sref3 = &*mref1;
    //     let ptr4 = sref3 as *const i32 as *mut i32;

    //     opaque_read(&*ptr4);
    //     opaque_read(sref3);
    //     *ptr2 += 2;
    //     *mref1 += 1;

    //     opaque_read(&data);
    // }
    //----------------------------------------------------------------

    //----------------------------------------------------------------
    unsafe {
        let mut data = 10;
        let mref1 = &mut data;
        let ptr2 = mref1 as *mut i32;
        let sref3 = &*mref1;

        *ptr2 += 2;
        opaque_read(sref3); // Read in the wrong order?
        *mref1 += 1;

        opaque_read(&data);
    }
    //----------------------------------------------------------------
}
