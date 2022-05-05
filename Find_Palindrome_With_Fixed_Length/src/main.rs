struct Solution;

impl Solution {
    fn nth(length: i32, n: i32) -> i64 {
        if length == 1 {
            if n == 0 {
                return 1;
            }
            return n as i64;
        }
        Solution::nth(length - 1, n / 10) * 10 + (n as i64 - 1) % 10
    }
    pub fn kth_palindrome(queries: Vec<i32>, int_length: i32) -> Vec<i64> {
        let mut ans = Vec::new();
        // for q in queries {
        //     ans.push(Solution::gen(int_length, q))
        // }
        ans
    }
}

fn main() {
    println!("{:?}", Solution::kth_palindrome(vec![1, 2, 3, 4, 5, 90], 3));
    println!("{:?}", Solution::nth(2, 10));
}
