struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut ranges = Vec::new();
        let mut left = None;
        let mut right = None;
        for n in nums {
            if let Some(r) = right {
                if r != n - 1 {
                    ranges.push(format!(
                        "{}->{}",
                        left.take().unwrap(),
                        right.take().unwrap()
                    ));
                    left = Some(n);
                    continue;
                }
                right = Some(n);
                continue;
            }
            if let Some(l) = left {
                if l != n - 1 {
                    ranges.push(format!("{}", left.take().unwrap()));
                    left = Some(n);
                    continue;
                }
                right = Some(n);
                continue;
            }
            left = Some(n);
        }
        if let Some(l) = left {
            if let Some(r) = right {
                ranges.push(format!("{}->{}", l, r,));
                return ranges;
            }
            ranges.push(format!("{}", left.take().unwrap()))
        }
        ranges
    }
}

fn main() {
    println!("Hello, world!");
}
