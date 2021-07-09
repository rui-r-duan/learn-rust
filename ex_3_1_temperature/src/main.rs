fn fahrenheit_to_celsius(v: f64) -> f64 {
    (v - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(v: f64) -> f64 {
    v * 9.0 / 5.0 + 32.0
}

fn main() {
    let f = 77.0;
    let c = 23.5;
    println!("{} \u{00b0}C = {} \u{00b0}F", fahrenheit_to_celsius(f), f);
    println!("{} \u{00b0}C = {} \u{00b0}F", c, celsius_to_fahrenheit(c));
}
