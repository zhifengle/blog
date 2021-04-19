mod duplication_in_array;
mod find_number_in2_d_array;

fn main() {
    println!("Hello, world!");
    let matrix: Vec<Vec<i32>> = vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ];
    println!("matrix: {}", matrix[1][0]);
}
