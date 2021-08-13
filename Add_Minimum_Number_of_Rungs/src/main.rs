struct Solution;

impl Solution {
    pub fn add_rungs(mut rungs: Vec<i32>, dist: i32) -> i32 {
        rungs.insert(0, 0);
        rungs
            .windows(2)
            .map(|w| {
                if w[1] - w[0] > dist {
                    if (w[1] - w[0]) % dist == 0 {
                        return (w[1] - w[0]) / dist - 1;
                    } else {
                        return (w[1] - w[0]) / dist;
                    }
                } else {
                    return 0;
                }
            })
            .sum()
    }
}

fn main() {
    println!("{}", Solution::add_rungs(vec![3, 4, 6, 7], 2));
}
