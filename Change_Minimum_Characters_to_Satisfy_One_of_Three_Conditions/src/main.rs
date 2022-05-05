struct Solution;

impl Solution {
    pub fn min_characters(a: String, b: String) -> i32 {
        let counts_a = a.chars().fold(vec![0; 26], |mut l, c| {
            l[c as usize - 97] += 1;
            l
        });
        let counts_b = b.chars().fold(vec![0; 26], |mut l, c| {
            l[c as usize - 97] += 1;
            l
        });
        let length = (a.len() + b.len()) as i32;
        let mut a_left_b_right = i32::MAX;
        let mut a_right_b_left = i32::MAX;
        let mut only_one = i32::MAX;
        for i in 1..26 {
            a_left_b_right = a_left_b_right.min(
                counts_a[..i].into_iter().sum::<i32>() + counts_b[i..].into_iter().sum::<i32>(),
            );
            a_right_b_left = a_right_b_left.min(
                counts_b[..i].into_iter().sum::<i32>() + counts_a[i..].into_iter().sum::<i32>(),
            );
        }
        for i in 0..26 {
            only_one = only_one.min(length - counts_a[i] - counts_b[i]);
        }
        a_left_b_right.min(a_right_b_left).min(only_one)
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_characters("dabadd".into(), "cda".into())
    );
}
