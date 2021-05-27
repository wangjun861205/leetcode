struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        nums.into_iter().for_each(|v| {
            let stack_len = stack.len();
            match stack_len {
                l if l == 0 => stack.push(v),
                l if l < k as usize => match stack.iter().position(|e| e <= &v) {
                    Some(idx) => stack.insert(idx, v),
                    None => stack.push(v),
                },
                _ => match stack.iter().position(|e| e <= &v) {
                    Some(idx) => {
                        stack.insert(idx, v);
                        stack.pop();
                    }
                    None => {
                        stack.push(v);
                        stack.pop();
                    }
                },
            }
        });
        stack.last().unwrap().clone()
    }
}
fn main() {
    println!("{}", Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2));
}
