struct Solution;

impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let mut sufsum: Vec<(i32, i32)> = nums
            .iter()
            .enumerate()
            .rev()
            .scan((0, 0), |(odd, even), (i, v)| {
                if i % 2 == 0 {
                    *even += *v;
                } else {
                    *odd += *v;
                }
                Some((*odd, *even))
            })
            .collect();
        sufsum.reverse();
        let mut ans = 0;
        let mut odd_sum = 0;
        let mut even_sum = 0;
        for (i, v) in nums.into_iter().enumerate() {
            if i % 2 == 0 {
                if odd_sum + sufsum[i].1 - v == even_sum + sufsum[i].0 {
                    ans += 1;
                }
            } else {
                if odd_sum + sufsum[i].1 == even_sum + sufsum[i].0 - v {
                    ans += 1;
                }
            }
            if i % 2 == 0 {
                even_sum += v;
            } else {
                odd_sum += v;
            }
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::ways_to_make_fair(vec![1, 2, 3]));
}
