struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn original_digits(s: String) -> String {
        let mut m: HashMap<char, i32> = s.chars().fold(HashMap::new(), |mut a, c| {
            *a.entry(c).or_insert(0) += 1;
            a
        });
        let mut numbers: Vec<char> = Vec::new();
        // "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        if let Some(count) = m.clone().get(&'z') {
            if count > &0 {
                numbers.append(&mut vec!['0'; count.clone() as usize]);
                *m.get_mut(&'z').unwrap() -= count.clone();
                *m.get_mut(&'e').unwrap() -= count.clone();
                *m.get_mut(&'r').unwrap() -= count.clone();
                *m.get_mut(&'o').unwrap() -= count.clone();
            }
        }
        // "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        if let Some(count) = m.clone().get(&'w') {
            if count > &0 {
                numbers.append(&mut vec!['2'; count.clone() as usize]);
                *m.get_mut(&'t').unwrap() -= count.clone();
                *m.get_mut(&'w').unwrap() -= count.clone();
                *m.get_mut(&'o').unwrap() -= count.clone();
            }
        }
        // "one", "three", "four", "five", "six", "seven", "eight", "nine",
        if let Some(count) = m.clone().get(&'u') {
            if count > &0 {
                numbers.append(&mut vec!['4'; count.clone() as usize]);
                *m.get_mut(&'f').unwrap() -= count.clone();
                *m.get_mut(&'o').unwrap() -= count.clone();
                *m.get_mut(&'u').unwrap() -= count.clone();
                *m.get_mut(&'r').unwrap() -= count.clone();
            }
        }
        // "one", "three", "five", "six", "seven", "eight", "nine",
        if let Some(count) = m.clone().get(&'x') {
            if count > &0 {
                numbers.append(&mut vec!['6'; count.clone() as usize]);
                *m.get_mut(&'s').unwrap() -= count.clone();
                *m.get_mut(&'i').unwrap() -= count.clone();
                *m.get_mut(&'x').unwrap() -= count.clone();
            }
        }
        // "one", "three", "seven", "eight", "nine",
        if let Some(count) = m.clone().get(&'f') {
            if count > &0 {
                numbers.append(&mut vec!['5'; count.clone() as usize]);
                *m.get_mut(&'f').unwrap() -= count.clone();
                *m.get_mut(&'i').unwrap() -= count.clone();
                *m.get_mut(&'v').unwrap() -= count.clone();
                *m.get_mut(&'e').unwrap() -= count.clone();
            }
        }
        // "one", "three", "eight", "nine",
        if let Some(count) = m.clone().get(&'s') {
            if count > &0 {
                numbers.append(&mut vec!['7'; count.clone() as usize]);
                *m.get_mut(&'s').unwrap() -= count.clone();
                *m.get_mut(&'e').unwrap() -= count.clone();
                *m.get_mut(&'v').unwrap() -= count.clone();
                *m.get_mut(&'e').unwrap() -= count.clone();
                *m.get_mut(&'n').unwrap() -= count.clone();
            }
        }
        // "one", "three", "nine",
        if let Some(count) = m.clone().get(&'g') {
            if count > &0 {
                numbers.append(&mut vec!['8'; count.clone() as usize]);
                *m.get_mut(&'e').unwrap() -= count.clone();
                *m.get_mut(&'i').unwrap() -= count.clone();
                *m.get_mut(&'g').unwrap() -= count.clone();
                *m.get_mut(&'h').unwrap() -= count.clone();
                *m.get_mut(&'t').unwrap() -= count.clone();
            }
        }
        // "three", "nine",
        if let Some(count) = m.clone().get(&'o') {
            if count > &0 {
                numbers.append(&mut vec!['1'; count.clone() as usize]);
                *m.get_mut(&'o').unwrap() -= count.clone();
                *m.get_mut(&'n').unwrap() -= count.clone();
                *m.get_mut(&'e').unwrap() -= count.clone();
            }
        }
        // "nine",
        if let Some(count) = m.clone().get(&'t') {
            if count > &0 {
                numbers.append(&mut vec!['3'; count.clone() as usize]);
                *m.get_mut(&'t').unwrap() -= count.clone();
                *m.get_mut(&'h').unwrap() -= count.clone();
                *m.get_mut(&'r').unwrap() -= count.clone();
                *m.get_mut(&'e').unwrap() -= count.clone();
                *m.get_mut(&'e').unwrap() -= count.clone();
            }
        }
        // empty
        if let Some(count) = m.clone().get(&'i') {
            if count > &0 {
                numbers.append(&mut vec!['9'; count.clone() as usize]);
                *m.get_mut(&'n').unwrap() -= count.clone();
                *m.get_mut(&'i').unwrap() -= count.clone();
                *m.get_mut(&'n').unwrap() -= count.clone();
                *m.get_mut(&'e').unwrap() -= count.clone();
            }
        }

        numbers.sort();
        numbers.into_iter().collect()
    }
}
fn main() {
    println!("{}", Solution::original_digits("owoztneoer".to_owned()));
}
