struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }
        let max_height = *height.iter().max().unwrap();
        let mut ans = 0;
        'outer: for h in 1..=max_height {
            let mut idx = 0;
            'mid: while idx < height.len() {
                if height[idx] < h {
                    idx += 1;
                } else {
                    for j in idx + 1..height.len() {
                        if height[j] >= h {
                            ans += j - idx - 1;
                            idx = j;
                            continue 'mid;
                        }
                    }
                    continue 'outer;
                }
            }
        }
        ans as i32
    }
}

fn main() {
    println!("{}", Solution::trap(vec![4, 2, 0, 3, 2, 5]));
}
