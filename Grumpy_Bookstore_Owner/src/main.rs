struct Solution;

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let length = customers.len();
        let mut presum: Vec<(i32, i32)> = customers
            .into_iter()
            .zip(grumpy)
            .scan((0, 0), |(norm, curb), (c, g)| {
                *norm += c * (g - 1).abs();
                *curb += c;
                Some((*norm, *curb))
            })
            .collect();
        presum.insert(0, (0, 0));
        let mut ans = 0;
        for i in 0..length {
            let mut sum = 0;
            sum += presum[i].0;
            let curb_right_index = i + minutes as usize;
            if curb_right_index < length + 1 {
                sum += presum[curb_right_index].1 - presum[i].1
            }
            if curb_right_index <= length {
                sum += presum[length].0 - presum[curb_right_index].0;
            }
            ans = ans.max(sum);
        }
        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_satisfied(vec![10, 1, 7], vec![0, 0, 0], 2)
    );
}
