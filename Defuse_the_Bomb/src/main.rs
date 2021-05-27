struct Solution;

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        code.clone()
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                if k > 0 {
                    (i + 1..=i + k as usize).map(|v| code[v % code.len()]).sum()
                } else if k < 0 {
                    (i as i32 + k..=i as i32 - 1)
                        .map(|j| code[(code.len() as i32 + j).abs() as usize % code.len()])
                        .sum()
                } else {
                    0
                }
            })
            .collect()
    }
}
fn main() {
    println!("{:?}", Solution::decrypt(vec![5, 7, 1, 4], 3));
}
