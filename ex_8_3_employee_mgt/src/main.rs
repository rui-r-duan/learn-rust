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
    BadSyntax,
    Exit,
}

fn main() {
    help();

    loop {
        prompt();

        let cmd = read_cmd();

        match eval_cmd(&cmd) {
            EvalResult::Good => continue,
            EvalResult::BadSyntax => {
                println!("Invalid command");
                help();
            }
            EvalResult::Exit => break,
        }
    }
}

fn prompt() {
    print!(">>> ");
    io::stdout().flush().expect("Failed to flush stdout");
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
                return eval_add_args(&mut iter);
            }
            "List" => {
                return eval_list_args(&mut iter);
            }
            "q" => {
                return EvalResult::Exit;
            }
            _ => {
                return EvalResult::BadSyntax;
            }
        },
        None => {
            return EvalResult::BadSyntax;
        }
    }
}

fn eval_add_args(iter: &mut SplitAsciiWhitespace) -> EvalResult {
    let employee: &str;
    match iter.next() {
	Some(x) => {
	    employee = x;
	}
	None => {
	    return EvalResult::BadSyntax;
	}
    }

    match iter.next() {
        Some(x) => {
            if x != "to" {
                return EvalResult::BadSyntax;
            }
        }
        None => {
            return EvalResult::BadSyntax;
        }
    }

    let department: &str;
    match iter.next() {
	Some(x) => {
	    department = x;
	}
	None => {
	    return EvalResult::BadSyntax;
	}
    }

    match iter.next() {
        Some(_) => {
            return EvalResult::BadSyntax;
        }
        None => ()
    }

    let mut map = HashMap::new(); // employee => department
    map.insert(employee, department);

    for (key, value) in &map {
        println!("{} in {}", key, value);
    }

    io::stdout().flush().expect("Failed to flush stdout");

    EvalResult::Good
}

fn eval_list_args(iter: &mut SplitAsciiWhitespace) -> EvalResult {
    match iter.next() {
        Some(x) => {
            println!("list all employees in department {}", x);
        }
        None => {
            println!("list all employees");
        }
    }

    EvalResult::Good
}
