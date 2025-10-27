use std::io;

fn fizzbuzz(t: i32) -> &'static str {
    if t % 3 == 0 {
        if t % 5 == 0 { "fizzbuzz" } else { "fizz" }
    } else if t % 5 == 0 {
        "buzz"
    } else {
        "zzz..."
    }
}

fn main() {
    let mut buf = String::new();
    loop {
        println!("Enter a number: ");
        io::stdin().read_line(&mut buf).unwrap();
        match buf.trim().parse::<i32>() {
            Ok(t) => {
                println!("{}", fizzbuzz(t));
            }
            Err(t) => {
                println!("Couldn't parse number {}! Enter a valid integer!", t);
            }
        }
        buf.clear();
        println!("Do you want to keep going? (Y/N): ");
        io::stdin().read_line(&mut buf).unwrap();
        if buf.trim().to_lowercase() != "y" {
            println!("Hope Fizzbuzz made you happy!");
            break;
        }
    }
}
