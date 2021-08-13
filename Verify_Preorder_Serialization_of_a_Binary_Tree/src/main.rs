struct Solution;

impl Solution {
    fn check(preorder: &mut Vec<String>, level: i32) -> bool {
        let c = preorder.remove(0);
        if c == "#" {
            if level == 0 {
                return preorder.is_empty();
            }
            return true;
        } else {
            if preorder.is_empty() {
                return false;
            }
            let left = Solution::check(preorder, level + 1);
            if preorder.is_empty() {
                return false;
            }
            let right = Solution::check(preorder, level + 1);
            if level == 0 && !preorder.is_empty() {
                return false;
            }
            return left && right;
        }
    }
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut preorder: Vec<String> = preorder.split(",").map(str::to_owned).collect();
        Solution::check(&mut preorder, 0)
    }
}
fn main() {
    println!("{}", Solution::is_valid_serialization("1,#".to_owned()));
}
