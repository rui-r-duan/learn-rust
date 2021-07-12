use std::io;
use std::collections::HashMap;

fn help() {
    println!("Usage:
* add an employee: Add <employee name> to <department name>
* retrieve people: List [department name]
");
}

fn main() {
    help();
    let mut cmd = String::new();

    io::stdin()
	.read_line(&mut cmd)
	.expect("Failed to read line");

    let mut iter = cmd.split_ascii_whitespace();
    match iter.next() {
	Some(x) => {
	    if x != "Add"
	    {
		help();
	    }
	}
	None => {
	    help();
	}
    }

    let employee = iter.next().unwrap();

    match iter.next() {
	Some(x) => {
	    if x != "to"
	    {
		help();
	    }
	}
	None => {
	    help();
	}
    }

    let department = iter.next().unwrap();

    let mut map = HashMap::new(); // employee => department
    map.insert(employee, department);

    for (key, value) in &map {
	println!("{} in {}", key, value);
    }
}
