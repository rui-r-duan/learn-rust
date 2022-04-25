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

    #[test]
    fn test_slice_of_slice() {
        let a = vec!["a", "bb", "ccc", "dddd"];
        // ["a", "bb", "ccc", "dddd"]
        println!("{:?}", a);

        let b = &a[0..3];
        // ["a", "bb", "ccc"]
        println!("{:?}", b);

        let bclone = b.clone();
        // ["a", "bb", "ccc"]
        println!("{:?}", bclone);

        // 0x7000024286c0, 0x700002428720
        println!(
            "{:?}, {:?}",
            std::ptr::addr_of!(b),
            std::ptr::addr_of!(bclone)
        );

        let addr_of_b: *const [&str] = b;
        let addr_of_bclone: *const [&str] = bclone;
        // 0x600000c88300, 0x600000c88300
        println!("{:?}, {:?}", addr_of_b, addr_of_bclone);

        let b_ptr = b.as_ptr();
        let bclone_ptr = bclone.as_ptr();
        // 0x600000c88300, 0x600000c88300
        println!("{:?}, {:?}", b_ptr, bclone_ptr);

        let c = &b[0..2];
        // ["a", "bb"]
        println!("{:?}", c);
    }
}
