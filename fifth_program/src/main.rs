use std::collections::HashMap;

#[macro_use]
extern crate text_io;

fn main() {
    let mut input = String::new();
    let mut integers: Vec<i32> = Vec::new();
    println!("Enter a list of integers: ");
    for i in 0..5 {
        println!("Int #{}: ", i + 1);
        input = read!();
        let trimmed = input.trim();
        match trimmed.parse::<i32>() {
            Ok(j) => {
                integers.push(j);
            }
            Err(..) => println!("input is not an integer: {}", trimmed),
        };
    }
    println!("The mean of all elements: {}", get_mean(&integers));
    println!("The median of the elements: {}", get_median(&integers));
    println!("The mode of the elements: {}", get_mode(&integers));
}

fn get_mean(array: &Vec<i32>) -> i32 {
    let mut mean: i32 = 0;
    for i in 0..array.len() {
        mean += array[i];
    }
    let mean = mean / (array.len() as i32);

    return mean;
}

fn get_median(array: &Vec<i32>) -> i32 {
    let mut median: i32 = 0;
    let mut ara: Vec<i32> = array.clone();
    ara.sort();
    median = ara[array.len() / 2];

    return median;
}

fn get_mode(array: &Vec<i32>) -> i32 {
    let mut mode: i32 = 0;
    let mut frequency_map: HashMap<i32, i32> = HashMap::new();

    for i in 0..array.len() {
        frequency_map
            .entry(array[i])
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    println!("{:?}", frequency_map);
    for i in 0..frequency_map.len() {
        // TODO: Make the logic!!!!!!!!
    }
    mode
}
