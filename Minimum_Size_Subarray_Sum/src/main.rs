struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut presum: Vec<i32> = nums
            .into_iter()
            .scan(0, |s, v| {
                *s += v;
                Some(*s)
            })
            .collect();
        presum.insert(0, 0);
        let mut left = 0_usize;
        let mut right = 0_usize;
        let mut ans = 0;
        while right < presum.len() {
            if presum[right] - presum[left] >= target {
                while left < right {
                    left += 1;
                    if presum[right] - presum[left] < target {
                        if ans == 0 {
                            ans = right - left + 1;
                        } else {
                            ans = ans.min(right - left + 1);
                        }
                        break;
                    }
                }
            }
            right += 1;
        }
        ans as i32
    }
}
fn main() {
    println!("{}", Solution::min_sub_array_len(4, vec![1, 4, 4]));
}
