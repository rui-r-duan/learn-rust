use std::collections::HashMap;
use std::io::{self, Write};
use std::str::SplitAsciiWhitespace;

fn help() {
    println!(
        "Usage:
* add an employee: Add <employee name> to <department name>
* retrieve people (two HashMap solution): List [department name]
* retrieve people (one HashMap solution): SlowList [department name]
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

    // department => employee
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
            "SlowList" => {
                return eval_slow_list_args(&mut iter, ctx);
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

fn eval_add_args(iter: &mut SplitAsciiWhitespace, ctx: &mut Context) -> EvalResult {
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

    // If there is any error in the execution, it must be unrecoverable, let it panic.
    exec_add(employee, department, ctx);

    println!("Done");
    println!("Employees Table = {:?}", ctx.employees);
    println!("Departments Table = {:?}", ctx.departments);

    EvalResult::Good
}

fn exec_add(employee: &str, department: &str, ctx: &mut Context) {
    if ctx.employees.contains_key(employee) {
        let old_dep = ctx.employees[employee].to_string();

        println!("{}'s old department: {}", employee, old_dep);
        println!("{}'s new department: {}", employee, department);

        // Overwrite the record in employees table
        ctx.employees
            .insert(employee.to_string(), department.to_string());

        // Cleanup the entry for the old department in Departments table
        match ctx.departments.get_mut(&old_dep) {
            Some(vec) => match vec.iter().position(|x| x == employee) {
                Some(i) => {
                    vec.swap_remove(i);
                    if vec.is_empty() {
                        ctx.departments.remove(&old_dep);
                    }
                }
                None => {
                    panic!("Error: Employee table says \"{}\" is in department \"{}\", but in the entry of \"{}\", the employee \"{}\" is not found", employee, old_dep, old_dep, employee);
                }
            },
            None => {
                panic!("Error: employee \"{}\"'s old department \"{}\" is not found in Department table", employee, old_dep);
            }
        }

        // Add the employee to the new department entry in Departments table
        let vec = ctx
            .departments
            .entry(department.to_string())
            .or_insert(Vec::new());
        vec.push(employee.to_string());
    } else {
        ctx.employees
            .insert(employee.to_string(), department.to_string());
        let vec = ctx
            .departments
            .entry(department.to_string())
            .or_insert(Vec::new());
        vec.push(employee.to_string());
    }
}

fn eval_list_args(iter: &mut SplitAsciiWhitespace, ctx: &mut Context) -> EvalResult {
    match iter.next() {
        Some(dep) => {
            println!("All employees in department {}:", dep);

            // O(1) to find the employee_list for the department
            match ctx.departments.get(dep) {
                Some(employee_list) => {
                    for elm in employee_list {
                        println!("{}", elm);
                    }
                }
                None => {
                    println!("Department {} is not found", dep);
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

fn eval_slow_list_args(iter: &mut SplitAsciiWhitespace, ctx: &mut Context) -> EvalResult {
    match iter.next() {
        Some(dep) => {
            println!("All employees in department {}:", dep);

            // O(N) to find the employee_list for the department, where N is
            // the length of the employees table
            let mut found = false;
            for (key, val) in &ctx.employees {
                if val == dep {
                    found = true;
                    println!("{}", key);
                }
            }
            if !found {
                println!("<<empty>>");
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
