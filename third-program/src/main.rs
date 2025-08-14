// use std::arch::x86_64::_SIDD_UWORD_OPS;

use std::fmt::format;

fn main() {
    let mut str = String::new();
    println!("Enter a string: ");
    std::io::stdin()
        .read_line(&mut str)
        .expect("Couldn't parse cmd!");

    println!("Pig Latin: {}", to_pig_latin(str.as_str()));
}

fn to_pig_latin(str: &str) -> String {
    let words = str.split(' ');
    let mut res: String = String::new();

    for word in words {
        let first_letter = word.chars().next().unwrap_or('\0');
        let full_word = word
            .split(first_letter)
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .to_string();
        let result = format(format_args!("{}-{}ay ", full_word, first_letter));
        res.push_str(result.as_str());
    }

    res
}
