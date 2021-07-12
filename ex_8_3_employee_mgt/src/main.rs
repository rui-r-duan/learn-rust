use std::collections::HashMap;
use std::io::{self, Write};
use std::str::SplitAsciiWhitespace;

fn help() {
    println!(
        "Usage:
* add an employee: Add <employee name> to <department name>
* retrieve people: List [department name]
* quit: q
"
    );
}

enum EvalResult {
    Good,
    Exit,
}

fn main() {
    help();

    loop {
        prompt();

        let cmd = read_cmd();

        match eval_cmd(&cmd) {
            EvalResult::Good => continue,
            EvalResult::Exit => break,
        }
    }
}

fn prompt() {
    print!(">>> ");
    io::stdout().flush().unwrap();
}

fn read_cmd() -> String {
    let mut cmd = String::new();

    io::stdin()
        .read_line(&mut cmd)
        .expect("Failed to read line");

    cmd
}

fn eval_cmd(cmd: &str) -> EvalResult {
    let mut iter = cmd.split_ascii_whitespace();
    match iter.next() {
        Some(x) => match x {
            "Add" => {
                eval_add_args(&mut iter);
            }
            "List" => {
                eval_list_args(&mut iter);
            }
            "q" => {
                return EvalResult::Exit;
            }
            _ => {
		println!("Invalid command");
                help();
            }
        },
        None => {
	    println!("Invalid command");
            help();
        }
    }
    EvalResult::Good
}

fn eval_add_args(iter: &mut SplitAsciiWhitespace) {
    let employee = iter.next().unwrap();

    match iter.next() {
        Some(x) => {
            if x != "to" {
		println!("Invalid command");
                help();
            }
        }
        None => {
	    println!("Invalid command");
            help();
        }
    }

    let department = iter.next().unwrap();

    let mut map = HashMap::new(); // employee => department
    map.insert(employee, department);

    for (key, value) in &map {
        println!("{} in {}", key, value);
    }

    io::stdout().flush().unwrap();
}

fn eval_list_args(iter: &mut SplitAsciiWhitespace) {
    match iter.next() {
        Some(x) => {
            println!("list all employees in department {}", x);
        }
        None => {
            println!("list all employees");
        }
    }
}
