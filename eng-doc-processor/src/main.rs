use std::{any::Any, collections::btree_map::RangeMut};

fn pr_sentence_case(string: &str) -> &str {
    let string = string.trim();
    let words = string.split(" ");

    let mut words: Vec<String> = words.into_iter().map(|x| x.to_string()).collect();
    words.iter().nth(0).unwrap().replace_range(
        0..=1,
        words[0]
            .chars()
            .nth(0)
            .unwrap()
            .to_ascii_uppercase()
            .to_string()
            .as_str(),
    );

    let res = words.join(" ");
    res.as_str()
}

fn main() {
    println!("Hello, world!");
}
