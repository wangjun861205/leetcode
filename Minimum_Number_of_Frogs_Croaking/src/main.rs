struct Solution;

impl Solution {
    pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
        let mut counts = vec![0; 5];
        let mut ans = 0;
        for c in croak_of_frogs.chars() {
            match c {
                'c' => {
                    counts[0] += 1;
                    ans = ans.max(counts[0] - counts[4]);
                }
                'r' => {
                    counts[1] += 1;
                    if counts[0] < counts[1] {
                        return -1;
                    }
                }
                'o' => {
                    counts[2] += 1;
                    if counts[1] < counts[2] {
                        return -1;
                    }
                }
                'a' => {
                    counts[3] += 1;
                    if counts[2] < counts[3] {
                        return -1;
                    }
                }
                'k' => {
                    counts[4] += 1;
                    if counts[3] < counts[4] {
                        return -1;
                    }
                }
                _ => unreachable!(),
            }
        }
        if counts.iter().all(|v| *v == counts[0]) {
            return ans;
        }
        -1
    }
}

fn main() {
    println!("{}", Solution::min_number_of_frogs("croakcroa".to_owned()));
}
