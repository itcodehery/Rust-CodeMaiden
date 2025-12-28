use std::process::exit;

fn roll_dice(global: &mut i32) {
    println!("Rolling....");
    let integer = 10;

    *global += integer;
}

fn view_score(global: &mut i32) {
    println!("-----------------------");
    println!("Score: {}", global);
    println!("-----------------------");
}

fn reset_score(global: &mut i32) {
    *global = 0;
    println!("-----------------------");
    println!("Score reset!!");
    println!("-----------------------");
}

fn main_menu(global: &mut i32) {
    let mut ch: String = String::new();
    println!("--------------------");
    println!("Dice Game Simulator");
    println!("--------------------");
    println!("1. Roll the Dice");
    println!("2. View Score");
    println!("3. Reset Score");
    println!("4. Exit");
    println!("--------------------");
    println!("Enter the number: ");
    std::io::stdin().read_line(&mut ch).unwrap();
    let ch = ch.trim();
    let ch = ch.parse::<i32>().unwrap();
    match ch {
        1 => roll_dice(global),
        2 => view_score(global),
        3 => reset_score(global),
        _ => exit(0),
    }
}

fn main() {
    let mut global: i32 = 0;
    loop {
        main_menu(&mut global);
    }
}
