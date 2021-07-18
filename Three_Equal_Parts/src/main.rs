struct Solution;

impl Solution {
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        let ones = arr.iter().filter(|&&v| v == 1).count();
        if ones == 0 {
            return vec![0, arr.len() as i32 - 1];
        }
        if ones % 3 != 0 {
            return vec![-1, -1];
        }
        let factor = ones / 3;
        let (mut s1, mut e1, mut s2, mut e2, mut s3, mut e3) = (0, 0, 0, 0, 0, 0);
        let mut count = 0;
        for i in 0..arr.len() {
            if arr[i] == 1 {
                count += 1;
                match count {
                    1 => {
                        s1 = i;
                        if factor == 1 {
                            e1 = i;
                        }
                    }
                    c if c == factor => e1 = i,
                    c if c == factor + 1 => {
                        s2 = i;
                        if factor == 1 {
                            e2 = i;
                        }
                    }
                    c if c == factor * 2 => e2 = i,
                    c if c == factor * 2 + 1 => {
                        s3 = i;
                        if factor == 1 {
                            e3 = i;
                        }
                    }
                    c if c == factor * 3 => e3 = i,
                    _ => {}
                }
            }
        }
        for ((v1, v2), v3) in arr[s1..e1 + 1]
            .iter()
            .zip(arr[s2..e2 + 1].iter())
            .zip(arr[s3..e3 + 1].iter())
        {
            if v1 != v2 || v2 != v3 {
                return vec![-1, -1];
            }
        }
        let tail_zero = arr.len() - 1 - e3;
        if s2 - e1 - 1 < tail_zero || s3 - e2 - 1 < tail_zero {
            return vec![-1, -1];
        }
        vec![(e1 + tail_zero) as i32, (e2 + tail_zero + 1) as i32]
    }
}
fn main() {
    println!("{:?}", Solution::three_equal_parts(vec![1, 1, 0, 0, 1]));
}
