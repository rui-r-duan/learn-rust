// This program prints the lyrics of the song "The Twelve Days of Christmas".

fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nignth",
        "tenth", "eleventh", "twelfth",
    ];
    let sentences = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    for i in 0..12 {
        println!(
            "One the {} day of Christmas my true love sent to me",
            days[i]
        );
        for j in (0..i + 1).rev() {
            println!("{}", sentences[j]);
        }
        println!();
    }
}
