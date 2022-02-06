struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut contains_zero = false;
        let groups: Vec<Vec<i32>> = nums
            .split(|v| {
                if v == &0 {
                    contains_zero = true;
                    return true;
                }
                false
            })
            .map(|v| v.to_vec())
            .filter(|l| !l.is_empty())
            .collect();
        let mut ans = i32::MIN;
        for l in groups {
            let prod = l.iter().product::<i32>();
            ans = ans.max(prod);
            if prod < 0 {
                let mut left = prod;
                for i in 0..l.len() - 1 {
                    left /= l[i];
                    if left > 0 {
                        break;
                    }
                }
                let mut right = prod;
                for i in (1..l.len()).rev() {
                    right /= l[i];
                    if right > 0 {
                        break;
                    }
                }
                if left != prod {
                    ans = ans.max(left);
                }
                if right != prod {
                    ans = ans.max(right);
                }
            }
        }
        if ans < 0 && contains_zero {
            return 0;
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::max_product(vec![-3, 0, 1, -2]));
}
