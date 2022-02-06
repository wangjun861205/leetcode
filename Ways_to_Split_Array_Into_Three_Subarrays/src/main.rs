struct Solution;

impl Solution {
    fn find_left_most(presum: &Vec<i32>, left: usize, right: usize, right_limit: i32) -> Option<usize> {
        let mut l = left;
        let mut r = right;
        while l < r {
            let mid = (l + r) / 2;
            if presum[right] - presum[mid] > right_limit {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        if presum[right] - presum[l] < presum[l] {
            return None;
        }
        Some(l)
    }

    fn find_right_most(presum: &Vec<i32>, left: usize, right: usize, right_limit: i32) -> Option<usize> {
        let mut l = left;
        let mut r = right;
        while l < r {
            let mid = (l + r) / 2;
            if presum[right] - presum[mid] < presum[mid] {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        if presum[right] - presum[l] > right_limit {
            return None;
        }
        Some(l)
    }

    pub fn ways_to_split(nums: Vec<i32>) -> i32 {
        let presum: Vec<i32> = nums
            .iter()
            .scan(0, |s, v| {
                *s += v;
                Some(*s)
            })
            .collect();
        let mut count: i128 = 0;
        for i in 2..nums.len() {
            if let Some(l) = Solution::find_left_most(&presum, 0, i - 1, presum[nums.len() - 1] - presum[i - 1]) {
                if let Some(r) = Solution::find_right_most(&presum, 0, i - 1, presum[nums.len() - 1] - presum[i - 1]) {
                    if l <= r {
                        count += (r - l) as i128;
                        count %= 1000000007;
                    }
                }
            }
        }
        count as i32
    }
}

fn main() {
    println!("{}", Solution::ways_to_split(vec![3, 2, 1]));
}
