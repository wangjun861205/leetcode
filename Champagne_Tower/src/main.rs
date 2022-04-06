struct Solution;

//                         1
//                     1       1
//                 1       1       1
//             1       1       1       1
//         1       1       1       1       1
//     1       1       1       1       1       1
// 1       1       1       1       1       1       1

//                         0
//                     1       2
//                 3       4        5
//             6       7       8        9
//         10      11      12       13      14
//     15      16      17      18       19      20
//21       22      23      24       25      26      27

impl Solution {
    fn to_index(row: i32, col: i32) -> usize {
        (0..row as usize + 1).sum::<usize>() + col as usize
    }
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let n = (1..=101).sum::<usize>();
        let mut tower = vec![0f64; n];
        tower[0] = poured as f64;
        for row in 0..100 {
            for col in 0..=row {
                let i = Solution::to_index(row, col);
                if tower[i] > 1f64 {
                    let half = (tower[i] - 1f64) / 2f64;
                    let ni = Solution::to_index(row + 1, col);
                    tower[ni] += half;
                    tower[ni + 1] += half;
                    tower[i] = 1f64;
                }
            }
        }
        let idx = Solution::to_index(query_row, query_glass);
        tower[idx]
    }
}

fn main() {
    println!("{}", Solution::champagne_tower(25, 6, 1));
}
