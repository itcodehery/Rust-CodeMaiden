use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use crate::passcode_helper::decrypt_message;

pub fn initialize_diary() -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true) // Create if doesn't exist
        .append(true) // Append to end (good for diary entries)
        .open("entries.txt")
        .expect("Failed to open or create diary file")
}

pub fn write_to_diary() {
    let mut content: String = String::new();
    let mut diary_file: File = initialize_diary();
    println!("--------------------------");
    println!("Enter the contents to be written into the diary: ");
    std::io::stdin()
        .read_line(&mut content)
        .expect("Error reading line!");

    let json_content: String = format!("> {}\n", &content);
    let mem_content: Vec<u8> = decrypt_message(json_content).into_bytes();
    diary_file
        .write_all(&mem_content)
        .expect("Error appending to file");
}

pub fn read_from_diary() -> String {
    let mut content: String = String::new();
    let mut diary_file: File = initialize_diary();
    println!("--------------------------");
    println!("Entering your diary...");
    println!("--------------------------");
    diary_file
        .read_to_string(&mut content)
        .expect("Error reading from diary!");

    return decrypt_message(content);
}
