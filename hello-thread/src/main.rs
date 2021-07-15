use std::thread;
use std::time::Duration;

fn main() {
    //----------------------------------------------------------------
    // Example 1
    //----------------------------------------------------------------
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // If we put handle.join() here, the main thread will wait for the spawned thread
    // to complete before continuing to the following loop.
    // handle.join().unwrap();

    //----------------------------------------------------------------
    // Example 2
    //----------------------------------------------------------------
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let v = vec![1, 2, 3];

    // without a "move" closure, error E0373
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // without a "move" closure, the borrow of `v` inside the spawned thread
    // may reference a non-existed Vec.
    // drop(v);                    // oh no!

    handle.join().unwrap();
}
