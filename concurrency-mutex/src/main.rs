use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Example 1
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    // Example 2
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // Error E0382 without Rc<T>
    // `counter` is moved into closure, in previous iteration of loop,
    // so the second iteration of loop cannot use `counter` any more.
    // Mutex needs to be owned by multiple threads using Rc<T>.
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        // Error E0277: Rc<Mutext<i32>> cannot be sent between threads safely,
        // because the trait `Send` is not implemented for `Rc<Mutex<i32>>`.
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
