struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut ans: HashSet<i32> = HashSet::new();
        if x == 1 && y == 1 {
            if bound < 2 {
                return Vec::new();
            } else {
                return vec![2];
            }
        } else if x == 1 {
            let y_exp = (bound as f32).log(y as f32) as i32 + 1;
            for i in 0..y_exp {
                let v = y.pow(i as u32) + 1;
                if v < bound {
                    ans.insert(v);
                }
            }
            return ans.into_iter().collect();
        } else if y == 1 {
            let x_exp = (bound as f32).log(x as f32) as i32 + 1;
            for i in 0..x_exp {
                let v = x.pow(i as u32) + 1;
                if v < bound {
                    ans.insert(v);
                }
            }
            return ans.into_iter().collect();
        } else {
            let x_exp = (bound as f32).log(x as f32) as i32 + 1;
            let y_exp = (bound as f32).log(y as f32) as i32 + 1;
            for i in (0..x_exp).rev() {
                for j in 0..y_exp {
                    let v = x.pow(i as u32) + y.pow(j as u32);
                    if v > bound {
                        break;
                    }
                    if v <= bound {
                        ans.insert(v);
                    }
                }
            }
            return ans.into_iter().collect();
        }
    }
}

fn main() {
    println!("{:?}", Solution::powerful_integers(2, 1, 10));
}
