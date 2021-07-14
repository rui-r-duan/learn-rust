use add_one;
// use add_two; // cargo build and run work without this "use"
use rand::Rng;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );
    println!(
        "Hello, world! {} plus two is {}!",
        num,
        add_two::add_two(num)
    );
    println!("Your random number is {}!", rand::thread_rng().gen_range(1..101));
}
