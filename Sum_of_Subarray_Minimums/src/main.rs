struct Solution;

impl Solution {
    pub fn sum_subarray_mins(mut arr: Vec<i32>) -> i32 {
        let l: Vec<i128> = arr.iter().map(|&v| v as i128).collect();
        let search_left = |i: usize| {
            let mut count = 0_i128;
            for v in arr[..i].iter().rev() {
                if v <= &arr[i] {
                    break;
                }
                count += 1;
            }
            return count + 1;
        };

        let search_right = |i: usize| {
            let mut count = 0;
            for v in arr[i + 1..].iter() {
                if v < &arr[i] {
                    break;
                }
                count += 1;
            }
            return count + 1;
        };
        let mut ans: i128 = 0;
        for i in 0..arr.len() {
            let left = search_left(i);
            let right = search_right(i);
            ans += left * right * l[i];
        }
        (ans % 1000000007) as i32
    }
}

fn main() {
    println!("{}", Solution::sum_subarray_mins(vec![11, 81, 94, 43, 3]));
}
