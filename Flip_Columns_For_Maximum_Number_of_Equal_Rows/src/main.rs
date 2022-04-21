struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let counts = matrix.into_iter().fold(HashMap::new(), |mut m, l| {
            let (normal, counter) =
                l.into_iter()
                    .fold((String::new(), String::new()), |(mut s1, mut s2), v| {
                        if v == 0 {
                            s1.push('0');
                            s2.push('1');
                        } else {
                            s1.push('1');
                            s2.push('0');
                        }
                        (s1, s2)
                    });
            *m.entry(normal).or_insert(0) += 1;
            *m.entry(counter).or_insert(0) += 1;
            m
        });
        *counts.values().into_iter().max().unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
