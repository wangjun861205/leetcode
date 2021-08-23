struct Solution;

impl Solution {
    pub fn min_deletion_size(mut strs: Vec<String>) -> i32 {
        let word_length = strs[0].len();
        let mut count = 0;
        'outer: for i in 0..word_length {
            let mut ss = strs.clone();
            let mut remain = Vec::new();
            while !ss.is_empty() {
                let s = ss.remove(0);
                if remain.is_empty() {
                    remain.push(s);
                } else {
                    let c = s.chars().nth(i).unwrap();
                    let p = remain.last().unwrap().chars().nth(i).unwrap();
                    if c < p {
                        count += 1;
                        continue 'outer;
                    } else if c == p {
                        remain.push(s);
                    } else {
                        remain.pop();
                        remain.push(s);
                    }
                }
            }
            if remain.len() == 1 {
                return count;
            }
            strs = remain;
        }
        count
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_deletion_size(
            vec!["xc", "yb", "za"]
                .into_iter()
                .map(str::to_owned)
                .collect()
        )
    );
}
