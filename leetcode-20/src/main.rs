// Leetcode-20: Valid Parenthesis
use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    // let mut stack: Vec<char> = Vec::new();
    // let str = s.chars();
    // let mut bracket_flag = true;

    // for i in str {
    //     if i == '(' || i == '{' || i == '[' {
    //         bracket_flag = false;
    //     }
    //     if i == ')' || i == '}' || i == ']' {
    //         bracket_flag = true;
    //     }
    // }
    //
    // New Idea: Hashmaps for each
    let mut hash_table: HashMap<char, i32> = HashMap::new();
    for i in s.chars() {
        hash_table.entry(i).and_modify(|x| *x += 1).or_insert(1);
    }
    println!("{:?}", hash_table);
    let mut bracket_flag = true;
    for char in s.chars() {
        bracket_flag = match char {
            ')' => hash_table.get(&'(').unwrap_or(&0) == hash_table.get(&')').unwrap_or(&0),
            ']' => hash_table.get(&'[').unwrap_or(&0) == hash_table.get(&']').unwrap_or(&0),
            '}' => hash_table.get(&'{').unwrap_or(&0) == hash_table.get(&'}').unwrap_or(&0),
            _ => false,
        }
    }

    bracket_flag
}

fn main() {
    println!("{}", is_valid(String::from("}{}{")));
}
