struct Solution;

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let prefix_sum: Vec<bool> = nums
            .iter()
            .scan(i32::MIN, |max, v| {
                *max = *max.max(&mut v.clone());
                Some(v < max)
            })
            .collect();
        let mut rev_prefix_sum: Vec<bool> = nums
            .iter()
            .rev()
            .scan(i32::MAX, |min, v| {
                *min = *min.min(&mut v.clone());
                Some(v > min)
            })
            .collect();
        rev_prefix_sum.reverse();
        if let Some(left) = rev_prefix_sum.into_iter().position(|v| v) {
            if let Some(right) = prefix_sum.into_iter().rposition(|v| v) {
                return (right - left + 1) as i32;
            }
            return 0;
        }
        0
    }
}

fn main() {
    println!("{}", Solution::find_unsorted_subarray(vec![1, 2, 3, 4]));
}
