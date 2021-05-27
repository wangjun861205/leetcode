struct Solution;

impl Solution {
    pub fn max_area(
        h: i32,
        w: i32,
        mut horizontal_cuts: Vec<i32>,
        mut vertical_cuts: Vec<i32>,
    ) -> i32 {
        horizontal_cuts.sort();
        vertical_cuts.sort();
        let mut row_dists: Vec<i32> = horizontal_cuts.windows(2).map(|w| w[1] - w[0]).collect();
        row_dists.insert(0, horizontal_cuts[0]);
        row_dists.push(h - *horizontal_cuts.last().unwrap());
        let mut col_dists: Vec<i32> = vertical_cuts.windows(2).map(|w| w[1] - w[0]).collect();
        col_dists.insert(0, vertical_cuts[0]);
        col_dists.push(w - *vertical_cuts.last().unwrap());
        (row_dists.into_iter().max().unwrap() as i64 * col_dists.into_iter().max().unwrap() as i64
            % (10_i64.pow(9) + 7)) as i32
    }
}
fn main() {
    println!("{}", Solution::max_area(5, 4, vec![3, 1], vec![1]));
}
