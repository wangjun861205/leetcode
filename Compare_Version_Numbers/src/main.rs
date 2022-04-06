struct Solution;

impl Solution {
    fn cmp(v1: &str, v2: &str) -> i32 {
        let n1 = v1.chars().rev().enumerate().fold(0, |mut s, (i, v)| {
            s += 10_i32.pow(i as u32) * v.to_digit(10).unwrap() as i32;
            s
        });
        let n2 = v2.chars().rev().enumerate().fold(0, |mut s, (i, v)| {
            s += 10_i32.pow(i as u32) * v.to_digit(10).unwrap() as i32;
            s
        });
        if n1 < n2 {
            return -1;
        } else if n1 > n2 {
            return 1;
        }
        0
    }
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut vs1: Vec<&str> = version1.split(".").collect();
        let mut vs2: Vec<&str> = version2.split(".").collect();
        if vs1.first().unwrap() == &"" {
            vs1.insert(0, "0");
        }
        if vs2.first().unwrap() == &"" {
            vs2.insert(0, "0");
        }
        let len_diff = (vs1.len() as i32 - vs2.len() as i32).abs();
        if vs1.len() < vs2.len() {
            vs1.append(&mut vec!["0"; len_diff as usize]);
        }
        if vs2.len() < vs1.len() {
            vs2.append(&mut vec!["0"; len_diff as usize]);
        }
        for (v1, v2) in vs1.into_iter().zip(vs2) {
            let res = Solution::cmp(v1, v2);
            if res != 0 {
                return res;
            }
        }
        0
    }
}

fn main() {
    println!("{}", Solution::compare_version("1.1".into(), "1.10".into()));
}
