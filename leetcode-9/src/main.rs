#[macro_use]
extern crate text_io;

pub fn is_palindrome(x: i32) -> bool {
    return x.to_string().chars().rev().eq(x.to_string().chars());
}

fn main() {
    println!("Enter a number: ");
    loop {
        let input: String = read!();
        let input = input.trim();
        match input.parse::<i32>() {
            Ok(j) => println!("Is {} a Palindrome?:  {}", j, is_palindrome(j)),
            Err(..) => {
                println!("Input is not an integer: {}", input);
                break;
            }
        }
    }
}
