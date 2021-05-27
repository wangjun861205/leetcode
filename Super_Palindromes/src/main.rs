struct Solution;

impl Solution {
    pub fn superpalindromes_in_range(left: String, right: String) -> i32 {
        let mut all: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        for i in 0..10000 {
            let mut s = i.to_string();
            s.push_str(&i.to_string().chars().rev().collect::<String>());
            all.push(s.parse::<i64>().unwrap());
            for j in 0..10 {
                let mut os = i.to_string();
                os.push_str(&j.to_string());
                os.push_str(&i.to_string().chars().rev().collect::<String>());
                all.push(os.parse::<i64>().unwrap());
            }
        }
        all.sort();
        let low_bound: i64 = left.parse::<i64>().unwrap();
        let high_bound: i64 = right.parse::<i64>().unwrap();
        let mut ans = 0;
        for n in all {
            let pow = n.pow(2);
            if pow > high_bound {
                break;
            }
            if pow < low_bound {
                continue;
            }
            let s = pow.to_string();
            if s == s.chars().rev().collect::<String>() {
                ans += 1;
            }
        }
        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::superpalindromes_in_range("1".to_owned(), "200".to_owned())
    );
}
