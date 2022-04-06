struct Solution;

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let (mut keep_top, mut keep_bottom, mut swap_top, mut swap_bottom) = (0, 0, 1, 1);
        for i in 1..tops.len() {
            if keep_top != -1 {
                if tops[i] != tops[0] && bottoms[i] != tops[0] {
                    keep_top = -1;
                }
                if keep_top != -1 && tops[i] != tops[0] {
                    keep_top += 1;
                }
            }
            if keep_bottom != -1 {
                if tops[i] != bottoms[0] && bottoms[i] != bottoms[0] {
                    keep_bottom = -1;
                }
                if keep_bottom != -1 && bottoms[i] != bottoms[0] {
                    keep_bottom += 1;
                }
            }
            if swap_top != -1 {
                if tops[i] != bottoms[0] && bottoms[i] != bottoms[0] {
                    swap_top = -1;
                }
                if swap_top != -1 && tops[i] != bottoms[0] {
                    swap_top += 1;
                }
            }
            if swap_bottom != -1 {
                if tops[i] != tops[0] && bottoms[i] != tops[0] {
                    swap_bottom = -1;
                }
                if swap_bottom != -1 && bottoms[i] != tops[0] {
                    swap_bottom += 1;
                }
            }
        }
        vec![keep_top, keep_bottom, swap_top, swap_bottom]
            .into_iter()
            .filter(|v| v >= &0)
            .min()
            .unwrap_or(-1)
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_domino_rotations(vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2],)
    );
}
