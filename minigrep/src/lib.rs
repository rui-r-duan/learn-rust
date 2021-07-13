use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
	if args.len() < 3 {
	    return Err("not enough arguments");
	}

	let query = args[1].clone();
	let filename = args[2].clone();

	Ok(Config { query: query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    // println!("With text:\n{}", contents);

    for line in search(&config.query, &contents) {
	println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
	if line.contains(query) {
	    results.push(line);
	}
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new_two_valid_args() {
	let args = vec![
	    String::from("program_name"),
	    String::from("contains?"),
	    String::from("filename.txt")];

	let config = Config::new(&args).unwrap();

	assert_eq!("contains?", config.query);
	assert_eq!("filename.txt", config.filename);
    }

    #[test]
    #[should_panic]
    fn config_new_less_args() {
	let args = vec!["a".to_string()];
	let config = Config::new(&args).unwrap();
    }

    #[test]
    fn one_result() {
	let query = "duct";
	let contents = "\
Rust:
safe, fast, productive.
Pick three.";

	assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
