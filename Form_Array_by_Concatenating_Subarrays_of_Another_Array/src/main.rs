struct Solution;

impl Solution {
    pub fn can_choose(groups: Vec<Vec<i32>>, mut nums: Vec<i32>) -> bool {
        let mut nums_index = 0_usize;
        for group in groups {
            let mut g_idx = 0;
            let mut n_idx = nums_index;
            while g_idx < group.len() {
                if n_idx == nums.len() {
                    return false;
                }
                if group[g_idx] == nums[n_idx] {
                    g_idx += 1;
                    n_idx += 1;
                } else {
                    g_idx = 0;
                    nums_index += 1;
                    n_idx = nums_index;
                }
            }
            nums_index = n_idx;
        }
        true
    }
}

fn main() {
    println!(
        "{}",
        Solution::can_choose(
            vec![vec![1, 2, 3], vec![3, 4]],
            vec![7, 7, 1, 2, 3, 4, 7, 7]
        )
    );
}
