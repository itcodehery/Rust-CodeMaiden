use std::io;

fn main() {
    let arr: Vec<i32> = vec![32, 43, 54, 12, 23, 56, 67, 87];
    println!("{:?}", arr);
    let mut key: String = String::new();
    println!("\nEnter the key to search: ");
    io::stdin().read_line(&mut key).unwrap();
    let key = key.trim();
    let key = match key.parse::<i32>() {
        Ok(t) => t,
        Err(_) => 0,
    };
    let found: u32 = linear_search(&arr, &key);
    if found == u32::MAX {
        println!("Element not found!");
    } else {
        println!("Found at {} ", found + 1);
    }
}

fn linear_search(arr: &Vec<i32>, key: &i32) -> u32 {
    for i in 0..arr.len() {
        if arr[i] == *key {
            return u32::try_from(i).unwrap();
        }
    }
    return u32::MAX;
}

fn binary_search(arr: &Vec<i32>, key: &i32) -> u32 {
    let mut mid = arr[arr.len() / 2];
}
