use std::collections::HashMap;

fn main() {
    let mut input: String = String::new();
    println!("Enter a string: ");
    std::io::stdin().read_line(&mut input).unwrap();

    println!("{:?}", word_count(input));
}

fn word_count(string: String) -> Result<HashMap<String, usize>, String> {
    if string.is_empty() {
        return Err(String::from("String is empty!"));
    }
    let mut result: HashMap<String, usize> = HashMap::new();

    for word in string.split_whitespace() {
        if result.keys().any(|x| x == word) {
            result.insert(word.to_string(), 1);
        }
    }

    return Ok(result);
}
