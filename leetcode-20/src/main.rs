// Leetcode-20: Valid Parenthesis
use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    let mut pairs: HashMap<char, char> = HashMap::new();
    pairs.insert('[', ']');
    pairs.insert('{', '}');
    pairs.insert('(', ')');

    let mut stack: Vec<char> = vec![];
    for char in s.chars() {
        match char {
            '[' => {
                stack.push(char);
            }
            '{' => {
                stack.push(char);
            }
            '(' => {
                stack.push(char);
            }
            ']' => {
                if stack.top {
                    stack.pop();
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }
    if stack.is_empty() { true } else { false }
}

fn main() {
    println!("{}", is_valid(String::from("(){}{[]}")));
}
