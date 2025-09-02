use std::io;

struct Pet {
    name: String,
    species: String,
    price: f64,
    is_trained: bool,
}

impl Pet {
    fn new() -> Pet {
        Pet {
            name: String::new(),
            species: String::new(),
            price: 0.0,
            is_trained: false,
        }
    }
}

fn main() {
    let mut x = String::from("Hello!");
    let y = &mut x;
    modify(y);
    println!("{}", y);

    // Struct prog
    let mut str = String::new();
    let mut new_pet = Pet::new();
    println!("Enter the details of the pet: ");

    println!("Name: ");
    io::stdin().read_line(&mut new_pet.name).unwrap();

    println!("Species: ");
    io::stdin().read_line(&mut new_pet.species).unwrap();

    println!("Price: ");
    io::stdin().read_line(&mut str).unwrap();
    let str = str.trim();
    new_pet.price = str.parse().unwrap_or(0.0);

    let mut str: String = String::from(str);
    println!("Is Trained? (yes/no)");
    io::stdin().read_line(&mut str).unwrap();

    let something = str.trim().to_lowercase();
    let str = something.as_str();
    match str {
        "yes" => new_pet.is_trained = true,
        "no" => new_pet.is_trained = false,
        _ => {
            println!("Invalid input. Defaulting to NO...");
            new_pet.is_trained = false
        }
    }
}

fn modify(param: &mut String) {
    (*param).push_str(" This is Hari!");
}
