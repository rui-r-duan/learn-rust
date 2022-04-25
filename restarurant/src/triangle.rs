use std::fmt;
use std::fmt::Result;
use std::io;
use std::io::Result as IoResult;

#[derive(Debug)]
struct Triangle {
    a: f32,
    b: f32,
    c: f32,
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result {
        write!(f, "({}, {}, {})", self.a, self.b, self.c)
    }
}

fn function1() -> String {
    let pythagorean_triple = Triangle {
        a: 3.0,
        b: 4.0,
        c: 5.0,
    };
    format!("{}", pythagorean_triple)
}

fn get_string() -> IoResult<String> {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer)?;

    Ok(buffer)
}

fn main() -> i32 {
    let secret_number = rand::thread_rng().gen_range(1..101);
    secret_number
}

use rand::Rng;

#[cfg(test)]
mod tests;
