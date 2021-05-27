struct Solution;

impl Solution {
    fn binary_search(nums: &Vec<i32>, target: i32, start: usize, end: usize) -> i32 {
        if start == end {
            if nums[start] == target {
                return start as i32;
            }
            return -1;
        }
        if end - start == 1 {
            if nums[start] == target {
                return start as i32;
            }
            if nums[end] == target {
                return end as i32;
            }
            return -1;
        }
        let mid = (start + end) / 2;
        let mid_val = nums[mid];
        let left_val = nums[start];
        let right_val = nums[end];
        if target == mid_val {
            return mid as i32;
        }
        if target == left_val {
            return start as i32;
        }
        if target == right_val {
            return end as i32;
        }
        if left_val < mid_val && mid_val < right_val {
            if target > left_val && target < mid_val {
                return Solution::binary_search(nums, target, start + 1, mid - 1);
            } else {
                return Solution::binary_search(nums, target, mid + 1, end - 1);
            }
        } else if left_val > mid_val && mid_val < right_val {
            if target > mid_val && target < right_val {
                return Solution::binary_search(nums, target, mid + 1, end - 1);
            } else {
                return Solution::binary_search(nums, target, start + 1, mid - 1);
            }
        } else if left_val < mid_val && mid_val > right_val {
            if target > left_val && target < mid_val {
                return Solution::binary_search(nums, target, start + 1, mid - 1);
            } else {
                return Solution::binary_search(nums, target, mid + 1, end - 1);
            }
        }
        -1
    }
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        Solution::binary_search(&nums, target, 0, nums.len() - 1)
    }
}
fn main() {
    println!("{}", Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
}
