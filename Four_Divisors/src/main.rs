struct Solution;

impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        'outer: for num in nums {
            let sqrt = (num as f64).sqrt() as i32 + 1;
            let mut count = 0;
            let (mut d1, mut d2) = (0, 0);
            for i in 2..sqrt {
                if num % i == 0 {
                    if num / i == i {
                        continue 'outer;
                    }
                    if count == 2 {
                        continue 'outer;
                    }
                    d1 = i;
                    d2 = num / i;
                    count = 2;
                }
            }
            if count == 0 {
                continue 'outer;
            }
            ans += d1 + d2 + num + 1;
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::sum_four_divisors(vec![21,]));
}
