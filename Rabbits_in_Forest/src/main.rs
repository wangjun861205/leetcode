struct Solution;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let counts = answers.into_iter().fold(vec![0; 1001], |mut l, v| {
            l[v as usize] += 1;
            l
        });
        let mut ans = 0;
        for i in 0..counts.len() {
            if counts[i] > 0 {
                if counts[i] > i as i32 + 1 {
                    let groups = if counts[i] % (i as i32 + 1) == 0 {
                        counts[i] / (i as i32 + 1)
                    } else {
                        counts[i] / (i as i32 + 1) + 1
                    };
                    ans += groups * (i as i32 + 1);
                } else {
                    ans += i as i32 + 1
                }
            }
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::num_rabbits(vec![0, 0, 1, 1, 1]));
}
