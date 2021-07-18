struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut s_to_t = vec![255_u8; 128];
        let mut t_to_s = vec![255_u8; 128];
        for (s, t) in s.chars().zip(t.chars()) {
            let su = s as u8;
            let tu = t as u8;
            if s_to_t[su as usize] == 255 && t_to_s[tu as usize] == 255 {
                s_to_t[su as usize] = tu;
                t_to_s[tu as usize] = su;
            } else {
                if s_to_t[su as usize] != tu || t_to_s[tu as usize] != su {
                    return false;
                }
            }
        }
        true
    }
}
fn main() {
    println!(
        "{}",
        Solution::is_isomorphic("egg".to_owned(), "add".to_owned())
    );
}
