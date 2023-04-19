use std::collections::HashSet;
use std::io;

fn is_token(t: &str) -> bool {
    t.chars().all(|ch| ch.is_ascii_digit())
        || t.contains(|ch: char| ch.is_ascii_digit())
            && t.contains(|ch: char| ch.is_ascii_alphabetic())
            && t.contains('-')
}
fn replace_token(t: &str) -> String {
    if is_token(t) {
        "{id}".to_string()
    } else {
        t.to_string()
    }
}

fn replace_tokens(path: &str) -> String {
    path.split('/')
        .map(replace_token)
        .collect::<Vec<String>>()
        .join("/")
}

fn main() {
    let lines = io::stdin().lines();
    let mut paths = HashSet::new();
    for line in lines {
        let input = line.unwrap();
        paths.insert(replace_tokens(&input));
    }
    for p in &paths {
        println!("{p}");
    }
}
