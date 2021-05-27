struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        for i in (0..nums.len()).rev() {
            for j in (i + 1..nums.len()).rev() {
                if nums[i] < nums[j] {
                    let mut ii = j as i32 - 1;
                    let mut jj = j;
                    while ii >= i as i32 {
                        nums.swap(ii as usize, jj as usize);
                        ii -= 1;
                        jj -= 1;
                    }
                    for k in i + 1..nums.len() {
                        for l in k + 1..nums.len() {
                            if nums[k] > nums[l] {
                                nums.swap(k, l);
                            }
                        }
                    }
                    return;
                }
            }
        }
        nums.sort();
    }
}
fn main() {
    let mut l = vec![4, 2, 0, 2, 3, 2, 0];
    Solution::next_permutation(&mut l);
    println!("{:?}", l);
}
