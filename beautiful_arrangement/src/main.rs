struct Solution;

impl Solution {
    pub fn process(start: i32, list: &mut Vec<bool>) -> i32 {
        if start == list.len() as i32 {
            return 1;
        }
        let mut count = 0;
        for i in 1..list.len() {
            if !list[i] && (start % i as i32 == 0 || i as i32 % start == 0) {
                list[i] = true;
                count += Solution::process(start + 1, list);
                list[i] = false;
            }
        }
        count
    }
    pub fn count_arrangement(n: i32) -> i32 {
        let mut l = vec![false; n as usize + 1];
        Solution::process(1, &mut l)
    }
}

fn main() {
    println!("{}", Solution::count_arrangement(7));
}
