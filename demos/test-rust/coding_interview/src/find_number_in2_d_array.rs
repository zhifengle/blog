#[allow(dead_code)]
fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let m = matrix.len();
    // 遗漏了  [], 0 的条件
    if m == 0 {
        return false;
    }
    // 遗漏了  [[]], 0 的条件
    let n = matrix[0].len();
    if n == 0 {
        return false;
    }
    let mut row = 0;
    let mut col = n - 1;
    while row < m  {
        if matrix[row][col] == target {
            return true;
        } else if matrix[row][col] > target {
            // 通过 row 来判断循环的。col 为 0 时
            if col == 0 {
                return false
            }
            col -= 1;
        } else {
            row += 1;
        }
    }

    false
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_number_in2_d_array_test() {
        let matrix: Vec<Vec<i32>> = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        assert_eq!(find_number_in2_d_array(matrix, 17), true);
        let m2 = vec![vec![-5]];
        assert_eq!(find_number_in2_d_array(m2, -10), false);
    }
}
