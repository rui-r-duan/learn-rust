use std::collections::HashMap;
use std::io::{self, Write};

fn help() {
    println!(
        "Usage (case sensitive):
* Add an employee: Add <employee name> to <department name>
* Retrieve people of a department (two HashMap solution): List <department name>
* Retrieve people of a department (one HashMap solution): SlowList <department name>
* Retrieve all people: List
* Quit: q | quit | exit
"
    );
}

enum EvalResult {
    Good,
    BadSyntax,
    Exit,
}

const ANSI_RED: &str = "\x1b[0;31m";
const ANSI_RESET: &str = "\x1b[0m";

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
                println!("{}", String::from(ANSI_RED) + "Invalid command" + ANSI_RESET);
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
    let tokens: Vec<String> = cmd
        .split_ascii_whitespace()
        .map(|s| s.to_string())
        .collect();

    if tokens.len() == 4 && tokens[0] == "Add" && tokens[2] == "to" {
        let employee = &tokens[1];
        let department = &tokens[3];

        // If there is any error in the execution, it must be unrecoverable,
        // let it panic.
        exec_add(employee, department, ctx)

    } else if tokens.len() == 2 && tokens[0] == "List" {
        let dep = &tokens[1];
	list_in_department(dep, ctx)

    } else if tokens.len() == 2 && tokens[0] == "SlowList" {
	let dep = &tokens[1];
	slow_list_in_department(dep, ctx)

    } else if tokens.len() == 1 && tokens[0] == "List" {
	list_all(ctx)

    } else if tokens.len() == 1 && (tokens[0] == "q" || tokens[0] == "quit" || tokens[0] == "exit")
    {
        EvalResult::Exit

    } else if tokens.len() == 0 {
        EvalResult::Good

    } else {
        EvalResult::BadSyntax
    }
}

fn exec_add(employee: &str, department: &str, ctx: &mut Context) -> EvalResult {
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
                    panic!("Error: Employee table says \"{}\" is in department \"{}\", \
			    but in the entry of \"{}\", the employee \"{}\" \
			    is not found", employee, old_dep, old_dep, employee);
                }
            },
            None => {
                panic!("Error: employee \"{}\"'s old department \"{}\" is not found \
			in Department table", employee, old_dep);
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

    println!("Done");
    println!("Employees Table = {:?}", ctx.employees);
    println!("Departments Table = {:?}", ctx.departments);
    io::stdout().flush().expect("Failed to flush stdout");

    EvalResult::Good
}

fn list_in_department(dep: &str, ctx: &Context) -> EvalResult {
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
    io::stdout().flush().expect("Failed to flush stdout");

    EvalResult::Good
}

fn slow_list_in_department(dep: &str, ctx: &Context) -> EvalResult {
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
    io::stdout().flush().expect("Failed to flush stdout");

    EvalResult::Good
}

fn list_all(ctx: &Context) -> EvalResult {
    println!("All employees:");
    for (key, val) in &ctx.employees {
        println!("{} in {}", key, val);
    }
    io::stdout().flush().expect("Failed to flush stdout");

    EvalResult::Good
}
