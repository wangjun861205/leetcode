struct Solution;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut freq = vec![0; k as usize];
        freq[0] = 1;
        let mut sum = 0;
        let mut count = 0;
        for n in nums {
            sum += n;
            let mut remain = sum % k;
            if remain < 0 {
                remain += k;
            }
            count += freq[remain as usize];
            freq[remain as usize] += 1;
        }
        count
    }
}

fn main() {
    println!(
        "{}",
        Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5)
    );
    println!("{}", -7 % 5);
}
