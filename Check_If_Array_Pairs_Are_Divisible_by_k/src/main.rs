struct Solution;

impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut freq = vec![0; k as usize];
        for v in arr {
            freq[(((v % k) + k) % k) as usize] += 1;
        }
        let mut left = 1_usize;
        let mut right = k as usize - 1;
        while left < right {
            if freq[left] != freq[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        if k % 2 == 0 {
            if freq[k as usize / 2] % 2 == 1 {
                return false;
            }
        }
        true
    }
}

fn main() {
    println!(
        "{}",
        Solution::can_arrange(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 5)
    );
    println!("{}", -5 % 3)
}
