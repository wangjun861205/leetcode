struct Solution;

impl Solution {
    pub fn str_without3a3b(mut a: i32, mut b: i32) -> String {
        let mut ans = Vec::new();
        while a > 0 || b > 0 {
            if a > b {
                if ans.is_empty() || ans.last().unwrap() == &'b' {
                    let n = a.min(2);
                    ans.append(&mut vec!['a'; n as usize]);
                    a -= n;
                } else {
                    ans.push('b');
                    b -= 1;
                }
            } else if a < b {
                if ans.is_empty() || ans.last().unwrap() == &'a' {
                    let n = b.min(2);
                    ans.append(&mut vec!['b'; n as usize]);
                    b -= n;
                } else {
                    ans.push('a');
                    a -= 1;
                }
            } else {
                if ans.is_empty() {
                    ans.append(&mut vec!['b', 'a']);
                    a -= 1;
                    b -= 1;
                } else if ans.last().unwrap() == &'a' {
                    ans.append(&mut vec!['b', 'a']);
                    a -= 1;
                    b -= 1;
                } else {
                    ans.append(&mut vec!['a', 'b']);
                    a -= 1;
                    b -= 1;
                }
            }
        }
        ans.into_iter().collect()
    }
}

fn main() {
    println!("{}", Solution::str_without3a3b(1, 1));
}
