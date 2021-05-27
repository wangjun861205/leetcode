struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();
        let mut prev: Option<i32> = None;
        let mut count = 0;
        for _ in 0..len {
            let n = nums.remove(0);
            if let Some(p) = prev {
                if p != n {
                    prev = Some(n);
                    nums.push(n);
                    count = 1;
                } else {
                    if count < 2 {
                        nums.push(n);
                        count += 1;
                    } else {
                        count += 1;
                    }
                }
            } else {
                prev = Some(n);
                nums.push(n);
                count = 1;
            }
        }
        nums.len() as i32
    }
}
fn main() {
    let mut l = vec![1, 1, 1, 2, 2, 2, 3, 3, 3];
    let len = Solution::remove_duplicates(&mut l);
    println!("{}", len);
    println!("{:?}", l);
}
