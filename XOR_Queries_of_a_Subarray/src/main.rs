struct Solution;

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut presum: Vec<i32> = arr
            .into_iter()
            .scan(0, |s, v| {
                *s ^= v;
                Some(*s)
            })
            .collect();
        presum.insert(0, 0);
        queries
            .into_iter()
            .map(|l| presum[l[1] as usize + 1] ^ presum[l[0] as usize])
            .collect()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::xor_queries(
            vec![1, 3, 4, 8],
            vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]]
        )
    );
}
