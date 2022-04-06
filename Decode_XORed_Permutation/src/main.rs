struct Solution;

impl Solution {
    pub fn decode(encoded: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; encoded.len() + 1];
        for i in 0..=ans.len() as i32 {
            ans[0] ^= i;
            if i < ans.len() as i32 && i % 2 == 1 {
                ans[0] ^= encoded[i as usize];
            }
        }
        for i in 1..ans.len() {
            ans[i] = ans[i - 1] ^ encoded[i - 1];
        }
        ans
    }
}

fn main() {
    println!("{:?}", Solution::decode(vec![6, 5, 4, 6]));
}
