struct Solution;

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut ans = 0;
        let mut sum = 0_i64;
        for v in nums {
            if sum >= n as i64 {
                break;
            }
            if v as i64 <= sum + 1 {
                sum += v as i64;
            } else {
                while sum + 1 < v as i64 {
                    sum += sum + 1;
                    ans += 1;
                    if sum >= n as i64 {
                        return ans;
                    }
                }
                sum += v as i64;
            }
        }
        while sum < n as i64 {
            sum += sum + 1;
            ans += 1;
        }
        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_patches(
            vec![1, 2, 2, 6, 34, 38, 41, 44, 47, 47, 56, 59, 62, 73, 77, 83, 87, 89, 94],
            20
        )
    );
}
