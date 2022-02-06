struct Solution;

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = is_water
            .into_iter()
            .map(|l| l.into_iter().map(|v| if v == 0 { -1 } else { 0 }).collect())
            .collect();
        loop {
            let mut filled = false;
            for r in 0..ans.len() as i32 {
                for c in 0..ans[0].len() as i32 {
                    if ans[r as usize][c as usize] != 0 {
                        let top = if r - 1 >= 0 {
                            ans[r as usize - 1][c as usize]
                        } else {
                            -1
                        };
                        let bottom = if r + 1 < ans.len() as i32 {
                            ans[r as usize + 1][c as usize]
                        } else {
                            -1
                        };
                        let left = if c - 1 >= 0 {
                            ans[r as usize][c as usize - 1]
                        } else {
                            -1
                        };
                        let right = if c + 1 < ans[0].len() as i32 {
                            ans[r as usize][c as usize + 1]
                        } else {
                            -1
                        };
                        if let Some(min) = vec![top, bottom, left, right]
                            .into_iter()
                            .filter(|v| v >= &0)
                            .min()
                        {
                            if ans[r as usize][c as usize] != min + 1 {
                                ans[r as usize][c as usize] = min + 1;
                                filled = true;
                            }
                        }
                    }
                }
            }
            if !filled {
                break;
            }
        }
        ans
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::highest_peak(vec![
            vec![0, 0, 0, 0, 0, 0, 1, 0],
            vec![0, 1, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 1, 0],
            vec![0, 0, 1, 0, 0, 0, 0, 0]
        ])
    );
    println!(
        "{:?}",
        Solution::highest_peak(vec![
            vec![0, 0, 0, 0, 0, 0, 1, 0],
            vec![0, 1, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 1, 0],
            vec![0, 0, 1, 0, 0, 0, 0, 0]
        ])
    );
}
