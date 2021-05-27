struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_operations_with_cache(
        nums: Vec<i32>,
        x: i32,
        m: &mut HashMap<(usize, i32), i32>,
    ) -> i32 {
        match nums.len() {
            0 => -1,
            1 => {
                if nums[0] == x {
                    1
                } else {
                    -1
                }
            }
            _ => {
                let len = nums.len();
                let sum = nums.iter().sum();
                if m.contains_key(&(len, sum)) {
                    return m.get(&(len, sum)).unwrap().clone();
                }
                let fnums: Vec<i32> = nums.clone().drain(1..).collect();
                let flen = fnums.len();
                let fsum: i32 = fnums.iter().sum();
                let lnums: Vec<i32> = nums.clone().drain(..len - 1).collect();
                let llen = lnums.len();
                let lsum: i32 = lnums.iter().sum();
                let first = nums.first().unwrap().clone();
                if x - first == 0 {
                    m.insert((len, sum), 1);
                    return 1;
                }
                if x - first < 0 {
                    m.insert((len, sum), -1);
                    return -1;
                }
                let last = nums.last().unwrap().clone();
                if x - last == 0 {
                    m.insert((len, sum), 1);
                    return 1;
                }
                if x - last < 0 {
                    m.insert((len, sum), -1);
                    return -1;
                }
                let fcount = Solution::min_operations_with_cache(
                    nums.clone().drain(1..).collect(),
                    x - first,
                    m,
                );
                let lcount = Solution::min_operations_with_cache(
                    nums.clone().drain(..len - 1).collect(),
                    x - last,
                    m,
                );
                if fcount == -1 && lcount == -1 {
                    m.insert((len, sum), -1);
                    return -1;
                }
                if fcount == -1 {
                    m.insert((len, sum), lcount + 1);
                    return lcount + 1;
                }
                if lcount == -1 {
                    m.insert((len, sum), fcount + 1);
                    return fcount + 1;
                }
                let min = fcount.min(lcount) + 1;
                m.insert((len, sum), min);
                min
            }
        }
    }

    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let mut m: HashMap<(usize, i32), i32> = HashMap::new();
        Solution::min_operations_with_cache(nums, x, &mut m)
    }
}

fn main() {
    println!("{}", Solution::min_operations(vec![3, 2, 20, 1, 1, 3], 10))
}
