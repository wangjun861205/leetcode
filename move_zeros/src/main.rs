struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let len = nums.len();
        let mut zero_count = 0;
        for _ in 0..len {
            let v = nums.remove(0);
            if v == 0 {
                zero_count += 1;
            } else {
                nums.push(v);
            }
        }
        nums.append(&mut vec![0; zero_count]);
    }
}

fn main() {
    let mut v = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut v);
    println!("{:?}", v);
}
