struct Solution;

impl Solution {
    pub fn sorted_squares(mut a: Vec<i32>) -> Vec<i32> {
        if a[0] < 0 {
            let mut left_index = 0_usize;
            let mut right_index = a.len() - 1;
            'outer: while left_index < right_index {
                while left_index < right_index {
                    if a[left_index].abs() > a[right_index].abs() {
                        a.swap(left_index, right_index);
                        right_index -= 1;
                        continue 'outer;
                    } else {
                        right_index -= 1;
                        continue;
                    }
                }
                left_index += 1;
                right_index = a.len() - 1;
            }
        }
        a.iter_mut().for_each(|v| *v = v.pow(2));
        a
    }
}

fn main() {
    println!("{:?}", Solution::sorted_squares(vec![-3, -3, -2, 1]));
}
