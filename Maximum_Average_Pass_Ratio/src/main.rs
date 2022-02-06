struct Solution;

use std::cmp::{Ord, PartialOrd};
use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq)]
struct Ratio(i32, i32);

impl Ord for Ratio {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let d1 = self.1 - self.0;
        let d2 = other.1 - other.0;
        if d1 == 0 || d2 == 0 {
            if d1 == 0 && d2 == 0 {
                return std::cmp::Ordering::Equal;
            } else if d1 == 0 {
                return std::cmp::Ordering::Less;
            }
            return std::cmp::Ordering::Greater;
        }
        if self.1 != other.1 {
            if self.1 < other.1 {
                return std::cmp::Ordering::Greater;
            } else {
                return std::cmp::Ordering::Less;
            }
        }
        if self.0 < other.0 {
            return std::cmp::Ordering::Greater;
        } else if self.0 == other.0 {
            return std::cmp::Ordering::Equal;
        }
        return std::cmp::Ordering::Less;
    }
}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, mut extra_students: i32) -> f64 {
        let mut heap: BinaryHeap<Ratio> = BinaryHeap::new();
        for c in classes {
            heap.push(Ratio(c[0], c[1]));
        }
        while extra_students > 0 {
            let mut c = heap.pop().unwrap();
            println!("{:?}", c);
            c.0 += 1;
            c.1 += 1;
            heap.push(c);
            extra_students -= 1;
        }
        let l = heap.into_vec();
        println!("{:?}", l);
        let length = l.len() as f64;
        let sum: f64 = l.into_iter().map(|v| v.0 as f64 / v.1 as f64).sum();
        sum / length
    }
}
fn main() {
    println!(
        "{}",
        Solution::max_average_ratio(vec![vec![2, 4], vec![3, 9], vec![4, 5], vec![2, 10]], 4)
    );
}
