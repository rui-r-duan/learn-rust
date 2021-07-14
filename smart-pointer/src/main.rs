use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// This implementation does not work.
// error[E0046]: not all trait items implemented, missing: `Target`
// impl<T> Deref for MyBox<T> {
//     fn deref(&self) -> &T {
//         &self.0
//     }
// }

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let w = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *w);          // *w <=> *(y.deref()) at compile time

    let m = MyBox::new(String::from("Rust"));
    hello(&m);                  // &m <=> &(*m)[..] at compile time

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("CustomSmartPointer for my stuff is created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer for other stuff is created.");
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
