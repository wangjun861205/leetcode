struct Solution;

// 1,
// 22,
// 122, 212, 221,
// 333,
// 1333, 3133, 3313, 3331,
// 22333, 23233, 23323, 23332, 32233, 32323, 32332, 33223, 33232, 33322,
// 4444,
// 14444, 41444, 44144, 44414, 44441,
// 224444, 242444, 244244, 244424, 244442, 422444, 424244, 424424, 424442, 442244, 442424, 442442, 444224, 444242, 444422,
// 155555, 515555, 551555, 555155, 555515, 555551,
// 666666,

//

use std::collections::HashSet;

impl Solution {
    fn shuffle(digits: Vec<i32>, exp: u32, curr: i32, set: &mut HashSet<i32>) {
        if digits.is_empty() {
            set.insert(curr);
        }
        for i in 0..digits.len() {
            let mut ds = digits.clone();
            let d = ds.remove(i);
            Solution::shuffle(ds, exp + 1, curr + 10_i32.pow(exp) * d, set);
        }
    }
    pub fn next_beautiful_number(n: i32) -> i32 {
        let starts = vec![
            vec![1],
            vec![2, 2],
            vec![1, 2, 2],
            vec![3, 3, 3],
            vec![1, 3, 3, 3],
            vec![2, 2, 3, 3, 3],
            vec![1, 2, 2, 3, 3, 3],
            vec![4, 4, 4, 4],
            vec![1, 4, 4, 4, 4],
            vec![2, 2, 4, 4, 4, 4],
            vec![1, 2, 2, 4, 4, 4, 4],
            vec![5, 5, 5, 5, 5],
            vec![1, 5, 5, 5, 5, 5],
            vec![2, 2, 5, 5, 5, 5, 5],
            vec![6, 6, 6, 6, 6, 6],
            vec![1, 6, 6, 6, 6, 6, 6],
        ];
        let mut all = HashSet::new();
        for s in starts {
            Solution::shuffle(s, 0, 0, &mut all);
        }
        let mut all: Vec<i32> = all.into_iter().collect();
        all.sort();
        for v in all {
            if v > n {
                return v;
            }
        }
        unreachable!()
    }
}

fn main() {
    println!("{}", Solution::next_beautiful_number(3000));
}
