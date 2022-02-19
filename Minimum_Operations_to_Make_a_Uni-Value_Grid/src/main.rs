struct Solution;

impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut l: Vec<i32> = grid.into_iter().flatten().collect();
        l.sort();
        let target = l[l.len() / 2];
        let mut ans = 0;
        for v in l {
            if v % x != target % x {
                return -1;
            }
            ans += (v - target).abs() / x;
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
