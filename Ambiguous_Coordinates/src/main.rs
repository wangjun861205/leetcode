struct Solution;

impl Solution {
    fn parse(chars: &[char]) -> Vec<String> {
        let mut strs: Vec<String> = Vec::new();
        if chars[0] == '0' {
            if chars.len() == 1 {
                strs.push("0".to_owned());
            } else {
                if chars.last().unwrap() != &'0' {
                    strs.push("0.".to_owned() + &chars[1..].iter().collect::<String>());
                }
            }
        } else {
            if chars.len() == 1 {
                strs.push(chars.iter().collect::<String>());
            } else {
                if chars.last().unwrap() == &'0' {
                    strs.push(chars.iter().collect::<String>());
                } else {
                    strs.push(chars.iter().collect::<String>());
                    for i in 0..chars.len() - 1 {
                        strs.push(
                            chars[..=i].iter().collect::<String>()
                                + "."
                                + &chars[i + 1..].iter().collect::<String>(),
                        );
                    }
                }
            }
        }
        strs
    }
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let mut chars: Vec<char> = s.chars().collect();
        chars.remove(0);
        chars.pop();
        let mut ans: Vec<String> = Vec::new();
        for i in 0..chars.len() - 1 {
            let left_strs = Solution::parse(&chars[..=i]);
            let right_strs = Solution::parse(&chars[i + 1..]);
            for l in left_strs.iter() {
                for r in right_strs.iter() {
                    ans.push(format!("({}, {})", l, r));
                }
            }
        }
        ans
    }
}
fn main() {
    println!("{:?}", Solution::ambiguous_coordinates("(123)".to_owned()));
}
