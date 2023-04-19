fn main() {
    let mut x = 4;

    // fn equal_to_x(z: i32) -> bool {
    // 	z == x
    // }
    let equal_to_x = |z| z == x;

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
    x = 6;			// E0506
    
    let y = 4;

    assert!(equal_to_x(y));
}
