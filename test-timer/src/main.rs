extern crate chrono;
extern crate timer;
use std::sync::mpsc::channel;

fn main() {
    let timer = timer::Timer::new();
    let (tx, rx) = channel();

    timer.schedule_with_delay(chrono::Duration::seconds(3), move || {
        tx.send(()).unwrap();
    });

    for received in rx {
        println!("{:?}", received);
    }
    println!("This code has been executed after 3 seconds");
}
