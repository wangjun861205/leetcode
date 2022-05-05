struct Solution;

use std::collections::HashMap;

impl Solution {
    fn dp(questions: &Vec<Vec<i32>>, index: usize, cache: &mut HashMap<usize, i64>) -> i64 {
        if index >= questions.len() {
            return 0;
        }
        let score = questions[index][0] as i64;
        let next_index = index + questions[index][1] as usize + 1;
        let solve = if let Some(c) = cache.get(&next_index) {
            *c
        } else {
            Solution::dp(questions, next_index, cache)
        } + score;
        let skip = if let Some(c) = cache.get(&(index + 1)) {
            *c
        } else {
            Solution::dp(questions, index + 1, cache)
        };
        let ans = solve.max(skip);
        cache.insert(index, ans);
        ans
    }
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        Solution::dp(&questions, 0, &mut HashMap::new())
    }
}

fn main() {
    println!(
        "{}",
        Solution::most_points(vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 3],
            vec![4, 4],
            vec![5, 5]
        ])
    );
}
