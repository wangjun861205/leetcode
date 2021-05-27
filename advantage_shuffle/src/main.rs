struct Solution;

impl Solution {
    pub fn advantage_count(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut a = a;
        a.sort();
        b.into_iter()
            .map(|v| {
                if let Some(i) = a.iter().position(|av| av > &v) {
                    a.remove(i)
                } else {
                    a.remove(0)
                }
            })
            .collect()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::advantage_count(
            vec![8, 2, 4, 4, 5, 6, 6, 0, 4, 7],
            //  [0, 2, 4, 4, 4, 5, 6, 6, 7, 8]
            vec![0, 8, 7, 4, 4, 2, 8, 5, 2, 0] //  [2, 0, 8, 5, 6, 4, 4, 6, 7, 4]
        )
    );
}
