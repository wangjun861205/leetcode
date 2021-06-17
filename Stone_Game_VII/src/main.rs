struct Solution;

impl Solution {
    fn dfs(
        stones: &Vec<i32>,
        i: usize,
        j: usize,
        dp: &mut Vec<Vec<i32>>,
        presum: &Vec<i32>,
    ) -> i32 {
        if i == j {
            return 0;
        }
        if dp[i][j] == 0 {
            let sum = presum[j + 1] - presum[i];
            dp[i][j] = (sum - stones[i] - Solution::dfs(stones, i + 1, j, dp, presum))
                .max(sum - stones[j] - Solution::dfs(stones, i, j - 1, dp, presum));
        }
        return dp[i][j];
    }
    pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; stones.len()]; stones.len()];
        let mut presum: Vec<i32> = stones
            .clone()
            .into_iter()
            .scan(0, |s, v| {
                *s += v;
                Some(*s)
            })
            .collect();
        presum.insert(0, 0);
        Solution::dfs(&stones, 0, stones.len() - 1, &mut dp, &presum)
    }
}

fn main() {
    println!("{}", Solution::stone_game_vii(vec![5, 3, 1, 4, 2]));
}
