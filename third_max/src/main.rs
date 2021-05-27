struct Solution;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut l: Vec<i32> = Vec::new();
        nums.into_iter().for_each(|v| match l.len() {
            0 => l.push(v),
            1 => {
                if v > l[0] {
                    l.insert(0, v);
                } else if v < l[0] {
                    l.push(v);
                }
            }
            2 => {
                if v > l[0] {
                    l.insert(0, v);
                } else if v < l[0] {
                    if v > l[1] {
                        l.insert(1, v);
                    } else if v < l[1] {
                        l.push(v);
                    }
                }
            }
            _ => {
                if v > l[0] {
                    l.insert(0, v);
                    l.remove(3);
                } else if v < l[0] {
                    if v > l[1] {
                        l.insert(1, v);
                        l.remove(3);
                    } else if v < l[1] {
                        if v > l[2] {
                            l.insert(2, v);
                            l.remove(3);
                        }
                    }
                }
            }
        });
        match l.len() {
            0 => panic!("empty array"),
            1 | 2 => l[0],
            _ => l[2],
        }
    }
}

fn main() {
    println!("{}", Solution::third_max(vec![2, 2, 3, 1]));
}
