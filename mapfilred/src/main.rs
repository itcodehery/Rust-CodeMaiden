fn main() {
    let new_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    // let fil_vec = filter(&mut new_vec, |x| x > 5);

    println!("The old vector: {:?}", new_vec);
    // println!("The filtered vector: {:?}", fil_vec);

    println!("The filter using a trait: {:?}", new_vec.filter(|x| x > 5));
}

trait Filt {
    fn filter<T>(self: &Self, function: T) -> Vec<i32>
    where
        T: Fn(i32) -> bool;
}

impl Filt for Vec<i32> {
    fn filter<T>(self: &Self, function: T) -> Vec<i32>
    where
        T: Fn(i32) -> bool,
    {
        let mut filtered_vec = Vec::new();
        for i in 0..self.len() {
            if function(self[i]) {
                filtered_vec.push(self[i]);
            }
        }
        filtered_vec
    }
}

// fn filter<T>(vec: &mut Vec<i32>, function: T) -> Vec<i32>
// where
//     T: Fn(i32) -> bool,
// {
//     let mut filtered_vec = Vec::new();
//     for i in 0..vec.len() {
//         if function(vec[i]) {
//             filtered_vec.push(vec[i]);
//         }
//     }
//     filtered_vec
// }
