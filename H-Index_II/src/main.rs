struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = citations.len() - 1;
        while l < r {
            let m = (l + r) / 2;
            let right_count = citations.len() - m - 1;
            let h = citations[m];
            if h > right_count as i32 {
                r = m;
            } else {
                l = m + 1;
            }
        }
        citations[l].min((citations.len() - l) as i32)
    }
}

fn main() {
    println!("{}", Solution::h_index(vec![1, 2]));
}
