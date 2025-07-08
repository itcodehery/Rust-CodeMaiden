use std::{
    fs::File,
    io::{self, BufReader, Read, Write},
};

fn main() {
    // TODO: Diary Management using File Operations and Basic Encryption.
    let my_diary: File = initialize_diary();
}

fn generate_hash(mut message: String) -> String {
    message = message;
    return message.to_string();
}

fn initialize_diary() -> File {
    let diary_file: File;
    // match uses pattern matching to check if a file named
    // entries.txt already exists.
    diary_file = match File::open("entries.txt") {
        Ok(file) => file,
        Err(_) => File::create("entries.txt").expect("Failed to create diary file"),
    };
    return diary_file;
}

fn initialize_passcode() -> File {
    // Initialize file
    let mut passcode_file: File;

    // Open the file or create one if it doesn't exist
    passcode_file = match File::open("obfs.txt") {
        Ok(file) => file,
        Err(_) => File::create("obfs.txt").expect("Couldn't create file!"),
    };

    // TODO: Create a buffered reader to read efficiently
    let file_contents = std::io::read_to_string(passcode_file).expect("Failed to read Passcode");

    // If the file has contents, ask the user to
    if file_contents.len() == 0 {
        let mut passcode = String::new();
        std::println!("Enter the password for your diary: ");
        std::io::stdin()
            .read_line(&mut passcode)
            .expect("Failed to read line!");
        passcode = generate_hash(passcode);
        passcode_file
            .write_all(passcode.as_bytes())
            .expect("Failed to write passcode into file");
    }

    return passcode_file;
}
