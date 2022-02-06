struct Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let mut odd = vec![0_i128; arr.len()];
        let mut even = vec![0_i128; arr.len()];
        if arr[0] % 2 == 0 {
            even[0] = 1;
        } else {
            odd[0] = 1;
        }
        for (i, n) in arr.into_iter().enumerate().skip(1) {
            if n % 2 == 0 {
                odd[i] = odd[i - 1];
                even[i] = even[i - 1] + 1;
            } else {
                odd[i] = even[i - 1] + 1;
                even[i] = odd[i - 1];
            }
        }
        (odd.into_iter().sum::<i128>() % 1000000007) as i32
    }
}

fn main() {
    println!("{}", Solution::num_of_subarrays(vec![2, 4, 6]));
}
