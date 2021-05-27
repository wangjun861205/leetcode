struct Solution;

impl Solution {
    fn rc(l: &mut Vec<char>, is_open: bool) -> (i32, bool) {
        let mut num = 0;
        while l.len() > 0 {
            let c = l.remove(0);
            if c == '(' {
                let (count, ok) = Solution::rc(l, true);
                if !ok {
                    return (num.max(count), false);
                } else {
                    num += count;
                }
            } else {
                if is_open {
                    return (num + 2, true);
                } else {
                    return (num, false);
                }
            }
        }
        (num, false)
    }
    pub fn longest_valid_parentheses(s: String) -> i32 {
        if &s == "" {
            return 0;
        }
        let mut chars: Vec<char> = s.chars().collect();
        let mut counts: Vec<i32> = Vec::new();
        let mut count = 0;
        while chars.len() > 0 {
            let c = chars.remove(0);
            if c == '(' {
                let (inner_count, ok) = Solution::rc(&mut chars, true);
                if ok {
                    count += count + inner_count;
                } else {
                    counts.push(count);
                    counts.push(inner_count);
                    count = 0;
                }
            } else {
                continue;
            }
        }
        counts.into_iter().max().unwrap()
    }
}
fn main() {
    println!(
        "{}",
        Solution::longest_valid_parentheses(")(())))(())())".to_owned())
    );
}
