struct Solution;

impl Solution {
    pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut swap = vec![nums1.len(); nums1.len()];
        let mut not_swap = vec![nums1.len(); nums1.len()];
        swap[0] = 1;
        not_swap[0] = 0;
        for i in 1..nums1.len() {
            if nums1[i - 1] < nums1[i] && nums2[i - 1] < nums2[i] {
                swap[i] = swap[i - 1] + 1;
                not_swap[i] = not_swap[i - 1];
            }
            if nums1[i - 1] < nums2[i] && nums2[i - 1] < nums1[i] {
                swap[i] = (not_swap[i - 1] + 1).min(swap[i]);
                not_swap[i] = swap[i - 1].min(not_swap[i]);
            }
        }
        (*swap.last().unwrap()).min(*not_swap.last().unwrap()) as i32
    }
}
fn main() {
    println!(
        "{}",
        Solution::min_swap(vec![0, 4, 4, 5, 9], vec![0, 1, 6, 8, 10])
    );
}
