struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut sorted = arr.clone();
        sorted.sort();
        // 原数组元素set
        let mut o_set: HashSet<i32> = HashSet::new();
        // 排序后数组元素set
        let mut s_set: HashSet<i32> = HashSet::new();
        let mut count = 0;
        for (o, s) in arr.into_iter().zip(sorted) {
            o_set.insert(o);
            s_set.insert(s);
            // 如果两个set中的元素相同，则证明这一部分可以作为单独的一个chunk， 计数加1, 然后清空两个set中的元素， 继续向下进行
            if o_set == s_set {
                count += 1;
                o_set.clear();
                s_set.clear();
            }
        }
        count
    }
}
fn main() {
    println!("{}", Solution::max_chunks_to_sorted(vec![1, 0, 2, 3, 4]));
}
