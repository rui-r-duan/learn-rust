use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        // E0382: borrow of moved value `vals`
        // println!("val is {:?}", vals);
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("MORE"),
            String::from("MESSAGES"),
            String::from("FOR"),
            String::from("YOU"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }

        // E0382: borrow of moved value `vals`
        // println!("val is {:?}", vals);
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
