struct Solution;

use std::collections::HashMap;

impl Solution {
    fn is_sub(s1: &str, s2: &str) -> bool {
        let mut chars1: Vec<char> = s1.chars().collect();
        let mut chars2: Vec<char> = s2.chars().collect();
        'outer: loop {
            if chars2.is_empty() {
                return true;
            }
            if chars1.is_empty() {
                return false;
            }
            let c2 = chars2.remove(0);
            while !chars1.is_empty() {
                let c1 = chars1.remove(0);
                if c1 == c2 {
                    continue 'outer;
                }
            }
            return false;
        }
    }
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        let mut l = vec![HashMap::new(); 10];
        for s in strs {
            *l[s.len() - 1].entry(s).or_insert(0) += 1;
        }
        let mut checked: Vec<String> = Vec::new();
        for m in l.into_iter().rev() {
            'mid: for (k, v) in m.iter() {
                if v == &1 {
                    for i in 0..checked.len() {
                        if Solution::is_sub(checked[i].clone().as_str(), k.as_str()) {
                            checked.push(k.clone());
                            continue 'mid;
                        }
                    }
                    return k.len() as i32;
                } else {
                    checked.push(k.clone());
                }
            }
        }
        -1
    }
}

fn main() {
    println!("{}", Solution::is_sub("aaa", "aa"));
    println!(
        "{}",
        Solution::find_lu_slength(
            vec!["aaa", "aaa", "aa"]
                .into_iter()
                .map(str::to_owned)
                .collect()
        )
    );
}
