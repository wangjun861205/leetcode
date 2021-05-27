struct Solution;

impl Solution {
    pub fn rc(ordered: Vec<char>, remain: Vec<char>) -> bool {
        if remain.len() == 0 {
            let s: String = ordered.iter().collect();
            if s.chars().nth(0).unwrap() == '0' {
                return false;
            }
            let n: i32 = s.parse().unwrap();
            return format!("{:b}", n).chars().filter(|c| c == &'1').count() == 1;
        }
        for (i, d) in remain.iter().enumerate() {
            let mut o = ordered.clone();
            o.push(*d);
            let mut r = remain.clone();
            r.remove(i);
            if Solution::rc(o, r) {
                return true;
            }
        }
        false
    }
    pub fn reordered_power_of2(n: i32) -> bool {
        let l: Vec<char> = n.to_string().chars().collect();
        Solution::rc(Vec::new(), l)
    }
}
fn main() {
    println!("{}", Solution::reordered_power_of2(10));
}
