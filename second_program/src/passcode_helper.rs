use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use crate::encryption_helper::{base64_decode, base64_encode};

pub fn encrypt_message(message: String) -> String {
    return base64_encode(&message);
}

pub fn decrypt_message(message: String) -> String {
    return base64_decode(&message).expect("Error decoding...");
}

pub fn initialize_passcode() -> File {
    let passcode_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true) // Create if doesn't exist
        .open("obfs.txt")
        .expect("Couldn't open or create file!");

    return passcode_file;
}

pub fn ask_passcode(mut passcode_file: File) -> bool {
    let mut passcode = String::new();

    // Check if the passcode file is empty by reading its contents into a buffer
    let mut check_buffer = Vec::new();
    passcode_file
        .read_to_end(&mut check_buffer)
        .expect("Failed to read passcode file!");

    let is_empty = check_buffer.is_empty();

    if is_empty {
        // First time setup
        println!("-----------------------------------");
        println!("--- Rustic Diary ---");
        println!("-----------------------------------");
        println!("Set up your diary with a password!");
        println!("Enter the password for your diary: ");
        std::io::stdin()
            .read_line(&mut passcode)
            .expect("Failed to read line!");
        let encrypted_passcode = encrypt_message(passcode);
        // Write the new passcode to the file
        passcode_file
            .write_all(encrypted_passcode.trim().as_bytes())
            .expect("Failed to write passcode to file!");

        return true; // Setup successful
    } else {
        // Existing passcode verification
        println!("-----------------------------------");
        println!("--- Rustic Diary ---");
        println!("-----------------------------------");
        println!("Enter your password: ");
        std::io::stdin()
            .read_line(&mut passcode)
            .expect("Failed to read passcode");

        if decrypt_message(passcode).trim() == String::from_utf8_lossy(&check_buffer).trim() {
            return true;
        } else {
            println!("Wrong passcode! Sorry!");
        }
    }

    return false;
}
