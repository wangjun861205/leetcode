struct Solution;

impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        let mut l: Vec<i32> = Vec::new();
        for i in 0..=n {
            match i {
                0 => l.push(0),
                1 => l.push(1),
                _ => {
                    if i % 2 == 0 {
                        l.push(l[i as usize / 2]);
                    } else {
                        l.push(l[i as usize / 2] + l[i as usize / 2 + 1]);
                    }
                }
            }
        }
        l.iter().max().unwrap().clone()
    }
}

fn main() {
    println!("{}", Solution::get_maximum_generated(7));
}
