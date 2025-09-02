fn main() {
    let vec: Vec<Vec<i32>> = vec![
        vec![1, 1, 0, 1, 0],
        vec![0, 0, 1, 0, 1],
        vec![0, 1, 1, 0, 1],
        vec![0, 0, 0, 0, 1],
        vec![1, 0, 0, 1, 1],
    ];

    let war = warshalls_algo(vec);
    let n = war.len();

    println!("{:?}", war);
}

fn warshalls_algo(input_matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = input_matrix.len();
    let mut closure = input_matrix.clone();

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if closure[i][k] == 1 && closure[k][j] == 1 {
                    closure[i][j] = 1;
                }
            }
        }
    }

    closure
}
