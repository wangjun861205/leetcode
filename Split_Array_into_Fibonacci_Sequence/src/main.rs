struct Solution;

impl Solution {
    fn check(l: &Vec<i32>) -> bool {
        l.len() >= 3 && l[l.len() - 3] + l[l.len() - 2] == l[l.len() - 1]
    }

    fn rc(chars: &Vec<char>, idx: usize, ans: &mut Vec<i32>) -> bool {
        if idx == chars.len() {
            return Solution::check(ans);
        }
        if chars[idx] == '0' {
            if ans.len() >= 2 {
                ans.push(0);
                if !Solution::check(ans) {
                    ans.pop();
                    return false;
                }
                return Solution::rc(chars, idx + 1, ans);
            } else {
                ans.push(0);
                if !Solution::rc(chars, idx + 1, ans) {
                    ans.pop();
                    return false;
                }
                return true;
            }
        }
        for i in idx..chars.len() {
            let n = chars[idx..=i]
                .into_iter()
                .collect::<String>()
                .parse::<i64>()
                .unwrap();
            if n > i32::MAX as i64 {
                break;
            }
            if ans.len() >= 2 {
                ans.push(n as i32);
                if !Solution::check(ans) {
                    ans.pop();
                    continue;
                }
                if !Solution::rc(chars, i + 1, ans) {
                    ans.pop();
                    continue;
                }
                return true;
            } else {
                ans.push(n as i32);
                if !Solution::rc(chars, i + 1, ans) {
                    ans.pop();
                    continue;
                }
                return true;
            }
        }
        false
    }
    pub fn split_into_fibonacci(num: String) -> Vec<i32> {
        let mut ans = Vec::new();
        Solution::rc(&num.chars().collect(), 0, &mut ans);
        ans
    }
}
fn main() {
    println!("{:?}", Solution::split_into_fibonacci("417420815174208193484163452262453871040871393665402264706273658371675923077949581449611550452755".into()));
}
