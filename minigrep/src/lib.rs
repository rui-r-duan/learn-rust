use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // Ownership and mutability are irrelevant.
    // type `env::Args` specifies the ownership of variable `args`.
    // `mut` specifies the mutability of variable `args`.
    //
    // There is no input lifetime parameter, so the lifetime elision rules don't apply.
    // So 'static is needed as the output lifetime.
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query: query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    // println!("With text:\n{}", contents);
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); // type: String, not &str slice
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Config {
        fn new_for_test<T>(mut args: T) -> Result<Config, &'static str>
        where
            T: Iterator<Item = String>,
        {
            args.next();

            let query = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a query string"),
            };

            let filename = match args.next() {
                Some(arg) => arg,
                None => return Err("Didn't get a file name"),
            };

            let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

            Ok(Config {
                query: query,
                filename,
                case_sensitive,
            })
        }
    }

    #[test]
    fn config_new_two_valid_args() {
        let args = vec![
            String::from("program_name"),
            String::from("contains?"),
            String::from("filename.txt"),
        ];

        let config = Config::new_for_test(args.into_iter()).unwrap();

        assert_eq!("contains?", config.query);
        assert_eq!("filename.txt", config.filename);
    }

    #[test]
    #[should_panic]
    #[allow(unused)]
    fn config_new_less_args() {
        let args = vec!["a".to_string()];
        let config = Config::new_for_test(args.into_iter()).unwrap();
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

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
