use std::collections::HashMap;

#[macro_use]
extern crate text_io;

fn main() {
    let mut emp_n_dept: HashMap<String, Vec<String>> = HashMap::new();
    let departments = vec!["Sales", "Engineering", "Marketing", "IT", "DevOps"];
    let mut ch: String = String::new();
    let mut emp_name: String = String::new();

    for i in departments {
        emp_n_dept.insert(i.to_string(), vec![]);
    }

    loop {
        println!("\nEnter your choice: ");
        println!("1.Add Employee to Sales");
        println!("2.Add Employee to Engineering");
        println!("3.Add Employee to Marketing");
        println!("4.Add Employee to IT");
        println!("5.Add Employee to DevOps");
        println!("6.View Employee in All Departments");
        println!("_.Exit");
        println!("Your choice: ");
        ch = read!();
        let ch = ch.trim();
        let choice: i32 = ch.parse().expect("Couldn't parse into integer!");
        if choice != 6 && choice > 0 && choice < 6 {
            print!("\nEnter the name of the Employee: ");
            emp_name = read!();
            let emp_name = emp_name.trim();
        }
        match choice {
            1 => {
                let vec: &Vec<String> = emp_n_dept.get("Sales").unwrap();
                let mut vec = vec.clone();
                vec.extend_from_slice(&[emp_name.to_string()]);
                emp_n_dept.insert(String::from("Sales"), vec);
            }
            2 => {
                let vec: &Vec<String> = emp_n_dept.get("Engineering").unwrap();
                let mut vec = vec.clone();
                vec.extend_from_slice(&[emp_name.to_string()]);
                emp_n_dept.insert(String::from("Engineering"), vec);
            }
            3 => {
                let vec: &Vec<String> = emp_n_dept.get("Marketing").unwrap();
                let mut vec = vec.clone();
                vec.extend_from_slice(&[emp_name.to_string()]);
                emp_n_dept.insert(String::from("Marketing"), vec);
            }
            4 => {
                let vec: &Vec<String> = emp_n_dept.get("IT").unwrap();
                let mut vec = vec.clone();
                vec.extend_from_slice(&[emp_name.to_string()]);
                emp_n_dept.insert(String::from("IT"), vec);
            }
            5 => {
                let vec: &Vec<String> = emp_n_dept.get("DevOps").unwrap();
                let mut vec = vec.clone();
                vec.extend_from_slice(&[emp_name.to_string()]);
                emp_n_dept.insert(String::from("DevOps"), vec);
            }
            6 => view_emp_map(&emp_n_dept),
            _ => {
                println!("Exiting...");
                break;
            }
        }
    }
}

fn view_emp_map(map: &HashMap<String, Vec<String>>) {
    println!("All Employees in Departments\n");
    for entry in map {
        println!("* {}", entry.0);
        for i in entry.1 {
            println!("  - {}", i);
        }
    }
}
