// Convert strings to pig latin.  The first consonant of each word is moved to
// the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words
// that start with a vowel have “hay” added to the end instead (“apple” becomes
// “apple-hay”).  Keep in mind the details about UTF-8 encoding!

use std::io;

fn is_vowel(c: &char) -> bool {
    const TABLE: &str = "aeiouAEIOU";
    for t in TABLE.chars() {
        if t == *c {
            return true;
        }
    }
    false
}

fn pig_latin(s: &str) -> String {
    let mut r = String::new();
    let mut chars = s.chars();
    let first_char = match chars.next() {
        Some(x) => x,
        None => {
            return r;
        }
    };

    if is_vowel(&first_char) {
        r += s;
        r += "-hay";
    } else {
        let remaining_s = chars.as_str();
        r.push_str(remaining_s);
        r += &'-'.to_string()[..]; // r.push('-');
        r.push(first_char);
        r.push_str("ay");
    }

    r
}

fn main() {
    let prompt = "\
================================================================
              Convert strings to pig latin.
================================================================
The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).

Please input a word:";

    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();
    let result = pig_latin(&input);
    println!("The Pig Latin of \"{}\" is \"{}\".", &input, &result);
}
