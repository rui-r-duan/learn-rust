use super::*;

#[test]
fn test_triangle_fmt() {
    assert_eq!(function1(), "(3, 4, 5)");
}

#[test]
fn test_main() {
    let x = main();
    assert!(x > 0);
    assert!(x < 101);
}
