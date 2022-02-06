struct Solution;

impl Solution {
    fn dp(mut list: Vec<i32>, idx: usize, remained: Vec<i32>) -> Vec<i32> {
        if idx == list.len() {
            if !remained.is_empty() {
                return Vec::new();
            }
            return list;
        }
        if list[idx] != 0 {
            return Solution::dp(list, idx + 1, remained);
        }
        for (i, v) in remained.clone().into_iter().enumerate() {
            if v == 1 {
                let mut next_list = list.clone();
                next_list[idx] = 1;
                let mut next_remained = remained.clone();
                next_remained.remove(i);
                let next = Solution::dp(next_list, idx + 1, next_remained);
                if !next.is_empty() {
                    return next;
                }
            } else {
                if idx + v as usize >= list.len() {
                    continue;
                }
                if list[idx + v as usize] != 0 {
                    continue;
                }
                let mut next_remained = remained.clone();
                next_remained.remove(i);
                let mut next_list = list.clone();
                next_list[idx] = v;
                next_list[idx + v as usize] = v;
                let next = Solution::dp(next_list, idx + 1, next_remained);
                if !next.is_empty() {
                    return next;
                }
            }
        }
        Vec::new()
    }
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let remained: Vec<i32> = (1..=n).rev().collect();
        let list = vec![0; n as usize * 2 - 1];
        Solution::dp(list, 0, remained)
    }
}

fn main() {
    println!("{:?}", Solution::construct_distanced_sequence(5));
}
