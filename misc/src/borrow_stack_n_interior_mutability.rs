#![allow(unused)]
use std::cell::{Cell, UnsafeCell};

fn opaque_read(val: &i32) {
    println!("{}", val);
}

fn main() {
    // Cell<T> wraps UnsafeCell<T>.  &Cell<T> really is no different from
    // &UnsafeCell<T> as far as aliasing is concerned.  &UnsafeCell<T> really
    // is no different from *mut T as far as aliasing is concerned.  So all of
    // mref1, ptr2, and sref3 are considered the same reborrow.
    //
    // Borrow Stack:
    // [mref(aka ptr2, sref3)]
    unsafe {
        let mut data = Cell::new(10);
        let mref1: &mut Cell<i32> = &mut data;
        let ptr2 = mref1 as *mut Cell<i32>;
        let sref3: &Cell<i32> = &*mref1;

        (*ptr2).set((*ptr2).get() + 2);
        sref3.set(sref3.get() + 3);
        (*ptr2).set((*ptr2).get() + 2);
        mref1.set(mref1.get() + 1);

        println!("{}", data.get());
    }

    // Borrow Stack:
    // [mref1(aka ptr2), sref3]
    unsafe {
        let mut data = UnsafeCell::new(10);
        let mref1: &mut i32 = data.get_mut(); // Get a mutable ref to the contents
        let ptr2 = mref1 as *mut i32;
        let sref3: &i32 = &*ptr2; // <6930> was created by a SharedReadOnly retag at offsets [0x0..0x4]

        *ptr2 += 2; // <6930> was later invalidated at offsets [0x0..0x4] by a write access
        opaque_read(sref3); // Undefined behavior: trying to retag from <6930> for SharedReadOnly permission at alloc3073[0x0], but that tag does not exist in the borrow stack for this location
        *mref1 += 1;

        println!("{}", *data.get());
    }

    // Borrow Stack:
    // [mref1(aka ptr3, sref3)]
    unsafe {
        let mut data = UnsafeCell::new(10);
        let mref1: &mut UnsafeCell<i32> = &mut data; // Mutable ref to the *outside*
        let ptr2: *mut i32 = mref1.get(); // Get a raw pointer to the insides
        let sref3: &UnsafeCell<i32> = &*mref1; // Get a shared ref to the *outside*

        *ptr2 += 2;
        opaque_read(&*sref3.get());
        *sref3.get() += 3;
        *mref1.get() += 1;

        println!("{}", *data.get());
    }

    // &UnsafeCell<T> really is no different from *mut T as far as aliasing is
    // concerned.
    // Borrow Stack:
    // [mref1(aka sref2, ptr3)]
    unsafe {
        let mut data = UnsafeCell::new(10);
        let mref1: &mut UnsafeCell<i32> = &mut data;
        // These two are swapped so the borrows are *definitely* totally stacked
        let sref2: &UnsafeCell<i32> = &*mref1;
        // Derive the ptr from the shared ref to be super safe!
        let ptr3: *mut i32 = sref2.get();

        *sref2.get() += 2;
        *ptr3 += 3;
        opaque_read(&*sref2.get());
        *sref2.get() += 2;
        *mref1.get() += 1;

        println!("{}", *data.get());
    }
}
