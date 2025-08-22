use std::ops::Deref;

fn main() {
    let new_vec = vec![
        String::from("Hery"),
        String::from("Arden"),
        String::from("Arnav"),
        String::from("Darshan"),
    ];
    println!("The old vector: {:?}", new_vec);
    println!(
        "The filter using a trait: {:?}",
        new_vec.filter(|x| x == "Hery")
    );

    println!(
        "Double of all elements: {:?}",
        new_vec.map(|x| x.to_string().push_str(" - Member"))
    );
}

// Define a trait for implementing Map, Filter and Reduce
trait MapFilRed<T> {
    fn filter<F>(&self, predicate: F) -> Vec<T>
    where
        F: Fn(&T) -> bool,
        T: Clone;

    fn reduce<F, U>(&self, initial: U, function: F) -> U
    where
        F: Fn(U, &T) -> U;

    fn map<F, U>(&self, function: F) -> Vec<U>
    where
        F: Fn(&T) -> U;
}

// Generic implementation for Vec<T>
impl<T> MapFilRed<T> for Vec<T> {
    fn filter<F>(&self, predicate: F) -> Vec<T>
    where
        F: Fn(&T) -> bool,
        T: Clone,
    {
        let mut filtered = Vec::new();
        for item in self {
            if predicate(item) {
                filtered.push(item.clone());
            }
        }
        filtered
    }

    fn reduce<F, U>(&self, initial: U, function: F) -> U
    where
        F: Fn(U, &T) -> U,
    {
        let mut acc = initial;
        for item in self {
            acc = function(acc, item);
        }
        acc
    }

    fn map<F, U>(&self, function: F) -> Vec<U>
    where
        F: Fn(&T) -> U,
    {
        let mut mapped = Vec::new();
        for item in self {
            mapped.push(function(item));
        }
        mapped
    }
}
