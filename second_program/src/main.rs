use std::fs::File;

mod diary_helper;
mod encryption_helper;
mod passcode_helper;

use crate::{
    diary_helper::{read_from_diary, write_to_diary},
    passcode_helper::{ask_passcode, initialize_passcode},
};

fn main() {
    let my_passcode: File = initialize_passcode();

    // Ask for Passcode
    if !(ask_passcode(my_passcode)) {
        return;
    }

    let mut option: String = String::new();
    let mut number: i8 = -1;

    while number >= -1 && number < 3 {
        option.clear(); // Clear the option or the read_line will append stuff
        std::println!("Welcome to your personal diary!");
        println!("1. Add a diary note");
        println!("2. View diary notes");
        println!("3. Exit");
        println!("\nSelect an option: ");
        std::io::stdin()
            .read_line(&mut option)
            .expect("Could not read line!");

        number = option.trim().parse().expect("Could not parse String!");
        match number {
            1 => {
                write_to_diary();
            }
            2 => {
                println!("{}", read_from_diary());
                println!("-----------------------")
            }
            _ => {
                println!("Exiting...");
                number = -2;
            }
        }
    }
}
