pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut missing_pos: i32 = 0;
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    let mut smallest = sorted_nums[0];
    if smallest < -10 {
        smallest = -10;
    }
    println!("Smallest: {}", smallest);
    let mut largest = sorted_nums[nums.len() - 1];
    if largest < smallest {
        largest = 0;
    }
    println!("Largest: {}", largest);

    for i in smallest..=largest {
        if !sorted_nums.contains(&i) {
            if sorted_nums.contains(&1) {
                if missing_pos < i && missing_pos > 0 {
                    return missing_pos;
                }
                missing_pos = i;
            } else {
                missing_pos = 1;
            }
        } else {
            if !sorted_nums.contains(&1) {
                missing_pos = 1;
            }
        }
    }
    if missing_pos == 0 {
        println!("Doesn't have one!");
        missing_pos = largest + 1;
    }
    missing_pos
}

fn main() {
    let vec: Vec<i32> = vec![7, 8, 9, 11, 12];
    println!("Hello, world!");
    println!(
        "The first missing positive: {}",
        first_missing_positive(vec)
    );
}
