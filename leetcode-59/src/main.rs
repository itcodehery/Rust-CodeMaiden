pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut merged_intervals: Vec<Vec<i32>> = Vec::new();
    let mut last_vec: Vec<i32> = intervals[0].clone();
    println!("Last vec: {:?}", last_vec);
    for i in 1..intervals.len() {
        if intervals[i][0] <= last_vec[1] {
            merged_intervals.push(vec![last_vec[0], intervals[i][1]]);
        } else {
            merged_intervals.push(intervals[i].clone());
        }
        last_vec = intervals[i].clone();
    }

    merged_intervals
}

fn main() {
    let intervals: Vec<Vec<i32>> = vec![vec![1, 4], vec![4, 5]];
    println!("Unmerged intervals: \n {:?}", intervals);

    let merged_intervals: Vec<Vec<i32>> = merge(intervals);
    println!("\nMerged intervals: \n {:?}", merged_intervals);
}
