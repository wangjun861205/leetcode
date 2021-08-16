struct Solution;

impl Solution {
    fn decode(s: &mut String) -> String {
        let mut ans = String::new();
        let mut times_str = String::new();
        while !s.is_empty() {
            let c = s.remove(0);
            if c.is_numeric() {
                times_str.push(c);
            } else if c == '[' {
                let next = Solution::decode(s);
                let times = times_str.parse::<usize>().unwrap();
                ans.push_str(&next.repeat(times));
                times_str.clear();
            } else if c == ']' {
                return ans;
            } else {
                ans.push(c);
            }
        }
        ans
    }
    pub fn decode_string(mut s: String) -> String {
        Solution::decode(&mut s)
    }
}

fn main() {
    println!("{}", Solution::decode_string("2[abc]3[cd]ef".to_owned()));
}
