struct Solution;

// [1, 2, 3, 4, 5, 6, 7] [-1, -1, -1, -1, -1, -1]
// [2, 1, 5, 7, 3, 6, 4] [1, -4, -2, 4, -3, 2]

impl Solution {
    pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
        let mut l: Vec<i32> = (1..=n).collect();
        let mut ans = Vec::new();
        if k % 2 != 0 {
            for i in 1..=k {
                if i % 2 != 0 {
                    ans.push(l.remove(0));
                } else {
                    ans.push(l.pop().unwrap());
                }
            }
        } else {
            for i in 1..=k {
                if i % 2 != 0 {
                    ans.push(l.pop().unwrap());
                } else {
                    ans.push(l.remove(0));
                }
            }
        }
        ans.append(&mut l);
        ans
    }
}
fn main() {
    println!("{:?}", Solution::construct_array(5, 1));
}
