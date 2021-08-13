struct Solution;

impl Solution {
    pub fn maximum_swap(mut num: i32) -> i32 {
        let mut nums = Vec::new();
        while num > 0 {
            let remain = num % 10;
            nums.insert(0, remain);
            num = num / 10;
        }
        let (mut max, mut max_idx) = (*nums.last().unwrap(), nums.len() - 1);
        let mut maxs = vec![(0, 0); nums.len()];
        for i in (0..nums.len()).rev() {
            if nums[i] > max {
                max = nums[i];
                max_idx = i;
            }
            maxs[i] = (max, max_idx);
        }
        'outer: for i in 0..nums.len() {
            let (max, max_idx) = maxs[i];
            if nums[i] < max {
                let temp = nums[i];
                nums[i] = max;
                nums[max_idx] = temp;
                break 'outer;
            }
        }
        nums.into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse()
            .unwrap()
    }
}

fn main() {
    println!("{}", Solution::maximum_swap(9973));
}
