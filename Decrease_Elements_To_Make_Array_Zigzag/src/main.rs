struct Solution;

impl Solution {
    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        let mut even_count = 0;
        let mut even = nums.clone();
        for i in 0..nums.len() {
            if i % 2 == 1 {
                let l = *even.get(i - 1).unwrap_or(&1001);
                let r = *even.get(i + 1).unwrap_or(&1001);
                let min = l.min(r);
                if even[i] >= min {
                    even_count += even[i] - min + 1;
                    even[i] = min - 1;
                }
            }
        }
        let mut odd_count = 0;
        let mut odd = nums.clone();
        for i in 0..nums.len() {
            if i == 0 {
                if odd[0] >= odd[1] {
                    odd_count += odd[0] - odd[1] + 1;
                    odd[0] = odd[1] - 1;
                }
                continue;
            }
            if i % 2 == 0 {
                let l = *odd.get(i - 1).unwrap_or(&1001);
                let r = *odd.get(i + 1).unwrap_or(&1001);
                let min = l.min(r);
                if odd[i] >= min {
                    odd_count += odd[i] - min + 1;
                    odd[i] = min - 1;
                }
            }
        }
        even_count.min(odd_count)
    }
}

fn main() {
    println!(
        "{}",
        Solution::moves_to_make_zigzag(vec![10, 4, 4, 10, 10, 6, 2, 3])
    );
}
