// Program: Armstrong Number
// Sum of Cube of all Digits should be equal to the Number
// itself

fn main() {
    let mut input: String = String::new();
    println!("Enter a number: ");
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let num = input.parse::<i32>().unwrap_or(0);
}
