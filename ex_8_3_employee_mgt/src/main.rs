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

struct Context {
    // employee => department
    employees: HashMap<String, String>,

    // department => demployee
    departments: HashMap<String, Vec<String>>,
}

fn main() {
    help();

    let mut ctx = Context {
        employees: HashMap::new(),
	departments: HashMap::new(),
    };

    loop {
        prompt();

        let cmd = read_cmd();

        match eval_cmd(&cmd, &mut ctx) {
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

fn eval_cmd(cmd: &str, ctx: &mut Context) -> EvalResult {
    let mut iter = cmd.split_ascii_whitespace();

    match iter.next() {
        Some(x) => match x {
            "Add" => {
                return eval_add_args(&mut iter, ctx);
            }
            "List" => {
                return eval_list_args(&mut iter, ctx);
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

fn eval_add_args(iter: &mut SplitAsciiWhitespace, ctx: & mut Context) -> EvalResult {
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
        None => (),
    }

    ctx.employees.insert(employee.to_string(), department.to_string());
    let vec = ctx.departments.entry(department.to_string()).or_insert(Vec::new());
    vec.push(employee.to_string());

    println!("Done");

    EvalResult::Good
}

fn eval_list_args(iter: &mut SplitAsciiWhitespace, ctx: &mut Context) -> EvalResult {
    match iter.next() {
        Some(dep) => {
            println!("All employees in department {}:", dep);
	    match ctx.departments.get(dep) {
		Some(employee_list) => {
		    for elm in employee_list {
			println!("{}", elm);
		    }
		}
		None => {
		    println!("No department {} is found", dep);
		}
	    }
        }
        None => {
            println!("All employees:");
	    for (key, val) in &ctx.employees {
		println!("{} in {}", key, val);
	    }
        }
    }

    io::stdout().flush().expect("Failed to flush stdout");

    EvalResult::Good
}
