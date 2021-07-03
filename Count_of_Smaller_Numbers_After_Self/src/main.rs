struct Solution;

impl Solution {
    fn sort(mut nums: Vec<(usize, i32)>, counts: &mut Vec<i32>) -> Vec<(usize, i32)> {
        match nums.len() {
            1 => return nums,
            2 => {
                if nums[0].1 > nums[1].1 {
                    counts[nums[0].0] += 1;
                    nums.swap(0, 1);
                }
                return nums;
            }
            _ => {
                let (left, right) = nums.split_at(nums.len() / 2);
                let (mut left, mut right) = (left.to_vec(), right.to_vec());
                left = Solution::sort(left, counts);
                right = Solution::sort(right, counts);
                let (mut i, mut j) = (0, 0);
                let mut l = Vec::new();
                let mut right_count = 0;
                while i < left.len() && j < right.len() {
                    if left[i].1 <= right[j].1 {
                        counts[left[i].0] += right_count;
                        l.push(left[i]);
                        i += 1;
                    } else {
                        right_count += 1;
                        l.push(right[j]);
                        j += 1;
                    }
                }
                while i < left.len() {
                    counts[left[i].0] += right_count;
                    l.push(left[i]);
                    i += 1;
                }
                while j < right.len() {
                    l.push(right[j]);
                    j += 1;
                }
                return l;
            }
        }
    }
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let l: Vec<(usize, i32)> = nums.into_iter().enumerate().collect();
        let mut counts = vec![0; len];
        Solution::sort(l, &mut counts);
        counts
    }
}

fn main() {
    println!("{:?}", Solution::count_smaller(vec![2, 0, 1]));
}
