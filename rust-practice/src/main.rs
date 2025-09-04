fn main() {
    let input = (false, true, false, true);
    println!("Input {:?}", input);
    let output = incrementer::incrementer(input.0, input.1, input.2, input.3);
    println!("Output {:?}", output);
}
