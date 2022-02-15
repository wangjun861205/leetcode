struct Solution;

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let mut results = vec![vec![i32::MAX; heights[0].len()]; heights.len()];
        results[0][0] = 0;
        loop {
            let mut updated = false;
            for i in 0..heights.len() {
                for j in 0..heights[0].len() {
                    if i > 0 {
                        let diff = (heights[i - 1][j] - heights[i][j])
                            .abs()
                            .max(results[i - 1][j]);
                        if diff < results[i][j] {
                            results[i][j] = diff;
                            updated = true;
                        }
                    }
                    if i < heights.len() - 1 {
                        let diff = (heights[i + 1][j] - heights[i][j])
                            .abs()
                            .max(results[i + 1][j]);
                        if diff < results[i][j] {
                            results[i][j] = diff;
                            updated = true;
                        }
                    }
                    if j > 0 {
                        let diff = (heights[i][j - 1] - heights[i][j])
                            .abs()
                            .max(results[i][j - 1]);
                        if diff < results[i][j] {
                            results[i][j] = diff;
                            updated = true;
                        }
                    }
                    if j < heights[0].len() - 1 {
                        let diff = (heights[i][j + 1] - heights[i][j])
                            .abs()
                            .max(results[i][j + 1]);
                        if diff < results[i][j] {
                            results[i][j] = diff;
                            updated = true;
                        }
                    }
                }
            }
            if !updated {
                break;
            }
        }
        *results.last().unwrap().last().unwrap()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::minimum_effort_path(vec![
            vec![1, 2, 1, 1, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 2, 1, 2, 1],
            vec![1, 1, 1, 2, 1]
        ])
    );
}
