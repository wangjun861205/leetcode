struct Solution;

const M: i64 = 1000000007;

impl Solution {
    fn dp(n: usize, visited: &mut Vec<bool>, cache: &mut Vec<Vec<i64>>) -> Vec<i64> {
        if n == 0 {
            return vec![1, 1, 1, 1, 1];
        }
        if visited[n - 1] {
            let prev = cache[n - 1].clone();
            cache[n][0] = (prev[1] + prev[2] + prev[4]) % M;
            cache[n][1] = (prev[0] + prev[2]) % M;
            cache[n][2] = (prev[1] + prev[3]) % M;
            cache[n][3] = prev[2] % M;
            cache[n][4] = (prev[2] + prev[3]) % M;
            visited[n] = true;
            return cache[n].clone();
        } else {
            let prev = Solution::dp(n - 1, visited, cache);
            cache[n][0] = (prev[1] + prev[2] + prev[4]) % M;
            cache[n][1] = (prev[0] + prev[2]) % M;
            cache[n][2] = (prev[1] + prev[3]) % M;
            cache[n][3] = prev[2] % M;
            cache[n][4] = (prev[2] + prev[3]) % M;
            visited[n] = true;
            return cache[n].clone();
        }
    }
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let mut visited = vec![false; n as usize];
        let mut cache = vec![vec![0; 5]; n as usize];
        let l = Solution::dp(n as usize - 1, &mut visited, &mut cache);
        (l.into_iter().sum::<i64>() % M) as i32
    }
}
fn main() {
    println!("{}", Solution::count_vowel_permutation(144));
}
