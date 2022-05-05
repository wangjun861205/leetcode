struct Solution;

impl Solution {
    pub fn num_splits(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut right: Vec<i32> = chars.iter().fold(vec![0; 26], |mut l, &v| {
            l[v as usize - 97] += 1;
            l
        });
        let mut left = vec![0; 26];
        let mut right_count = right.iter().filter(|v| v > &&0).count();
        let mut left_count = 0;
        let mut ans = 0;
        for i in 0..chars.len() {
            let c = chars[i];
            left[c as usize - 97] += 1;
            right[c as usize - 97] -= 1;
            if left[c as usize - 97] == 1 {
                left_count += 1;
            }
            if right[c as usize - 97] == 0 {
                right_count -= 1;
            }
            if left_count == right_count {
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::num_splits("aacaba".into()));
}
