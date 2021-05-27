struct Solution;

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let mut lens = vec![0; nums.len()];
        let mut counts = vec![0; nums.len()];
        lens[0] = 1;
        counts[0] = 1;
        for i in 1..nums.len() {
            let mut max = 0;
            let mut count = 0;
            for j in (0..i).rev() {
                if nums[i] > nums[j] {
                    if lens[j] > max {
                        max = lens[j];
                        count = counts[j];
                    } else if lens[j] == max {
                        count += counts[j];
                    }
                }
            }
            if max > 0 {
                lens[i] = max + 1;
                counts[i] = count;
            } else {
                lens[i] = 1;
                counts[i] = 1;
            }
        }
        println!("lens: {:?}", lens);
        println!("counts: {:?}", counts);
        let max = *lens.iter().max().unwrap();
        lens.into_iter()
            .zip(counts)
            .filter(|(l, _)| l == &max)
            .map(|(_, c)| c)
            .sum()
    }
}
fn main() {
    println!("{}", Solution::find_number_of_lis(vec![2, 2, 2, 2, 2]));
}
