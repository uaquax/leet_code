struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        if row_index < 1 {
            return vec![1];
        }

        let mut result: Vec<Vec<i32>> = vec![];

        for i in 0..row_index + 1 {
            let mut row = vec![];

            for _ in 0..i + 1 {
                row.push(0);
            }
            result.push(row);
            for j in 0..i + 1 {
                if j == 0 || j == i || i < 2 {
                    result[i as usize][j as usize] = 1;
                } else {
                    result[i as usize][j as usize] =
                        result[i as usize - 1][j as usize] + result[i as usize - 1][j as usize - 1];
                }
            }
        }

        result.pop().unwrap()
    }
}

fn main() {
    let result = Solution::get_row(0);
    println!("Result: {:?}", result);
}
