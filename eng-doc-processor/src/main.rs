use std::{io, str::Chars};

fn to_sentence_case(str: &mut String) -> String {
    let mut temp = str.as_mut_str();
    let chararr: Chars = temp.chars();
    for ref mut character in chararr {
        *character = character.to_ascii_uppercase();
        break;
    }
}

fn main() {
    let mut string: String = String::new();
    println!("Enter a sentence: ");
    io::stdin().read_line(&mut string).unwrap();

    println!("\n{}", to_sentence_case(&mut string));
}
