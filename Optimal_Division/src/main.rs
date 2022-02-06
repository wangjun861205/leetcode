struct Solution;

use std::collections::HashMap;

impl Solution {
    fn gen_exp(nums: &[f64]) -> String {
        let mut s = nums[0].to_string();
        for i in 1..nums.len() {
            s.push('/');
            s.push_str(&nums[i].to_string());
        }
        s
    }
    fn calc_div(nums: &[f64]) -> f64 {
        let mut div = nums[0];
        for i in 1..nums.len() {
            div = div / nums[i];
        }
        div
    }

    fn find_max(nums: Vec<f64>, max_cache: &mut HashMap<usize, (f64, String)>, min_cache: &mut HashMap<usize, (f64, String)>) -> (f64, String) {
        if nums.len() == 0 {
            return (1.0, "".to_string());
        }
        if nums.len() == 1 {
            return (nums[0] as f64, nums[0].to_string());
        }
        let mut value: f64 = Solution::calc_div(&nums[..]);
        let mut s = format!("({})", Solution::gen_exp(&nums));
        for i in 1..nums.len() {
            let head_v = Solution::calc_div(&nums[..i]);
            let head_s = Solution::gen_exp(&nums[..i]);
            let (nv, ns) = if let Some((v, s)) = min_cache.get(&(nums.len() - i)) {
                (*v, s.clone())
            } else {
                Solution::find_min(nums[i..].to_vec().clone(), max_cache, min_cache)
            };
            let v = head_v / nv;
            if value < v {
                value = v;
                s = format!("({}/{})", head_s, ns);
            }
        }
        max_cache.insert(nums.len(), (value, s.clone()));
        (value, s)
    }
    fn find_min(nums: Vec<f64>, max_cache: &mut HashMap<usize, (f64, String)>, min_cache: &mut HashMap<usize, (f64, String)>) -> (f64, String) {
        if nums.len() == 0 {
            return (1.0, "".to_string());
        }
        if nums.len() == 1 {
            return (nums[0] as f64, nums[0].to_string());
        }
        let mut value: f64 = Solution::calc_div(&nums[..]);
        let mut s = format!("({})", Solution::gen_exp(&nums));
        for i in 1..nums.len() {
            let head_v = Solution::calc_div(&nums[..i]);
            let head_s = Solution::gen_exp(&nums[..i]);
            let (nv, ns) = if let Some((v, s)) = max_cache.get(&(nums.len() - i)) {
                (*v, s.clone())
            } else {
                Solution::find_max(nums[i..].to_vec().clone(), max_cache, min_cache)
            };
            let v = head_v / nv;
            if value > v {
                value = v;
                s = format!("({}/{})", head_s, ns);
            }
        }
        min_cache.insert(nums.len(), (value, s.clone()));
        (value, s)
    }
    pub fn optimal_division(nums: Vec<i32>) -> String {
        if nums.len() == 1 {
            return nums[0].to_string();
        }
        let nums = nums.into_iter().map(|v| v as f64).collect();
        let (_, mut s) = Solution::find_max(nums, &mut HashMap::new(), &mut HashMap::new());
        s = (&s[1..s.len() - 1]).into();
        s
    }
}

fn main() {
    println!("{}", Solution::optimal_division(vec![1000, 100, 10, 2]));
}
