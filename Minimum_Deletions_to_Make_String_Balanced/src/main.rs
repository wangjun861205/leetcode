struct Solution;

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut a_count = 0;
        let mut b_count = 0;
        let mut remain_a: Vec<i32> = s
            .chars()
            .rev()
            .scan(0, |l, v| {
                if v == 'a' {
                    a_count += 1;
                    *l += 1;
                    return Some(*l);
                }
                b_count += 1;
                Some(*l)
            })
            .collect();
        remain_a.reverse();
        let mut ans = i32::MAX;
        let mut removed_b = 0;
        for (i, c) in s.chars().enumerate() {
            if c == 'b' {
                ans = ans.min(removed_b + remain_a[i]);
                removed_b += 1;
            }
        }
        if ans == i32::MAX {
            return 0;
        }
        ans.min(a_count).min(b_count)
    }
}

fn main() {
    println!("{}", Solution::minimum_deletions("aabbbbaabababbbbaaaaaabbababaaabaabaabbbabbbbabbabbababaabaababbbbaaaaabbabbabaaaabbbabaaaabbaaabbbaabbaaaaabaa".to_owned()));
}
