struct Solution;

impl Solution {
    pub fn min_eating_speed(mut piles: Vec<i32>, h: i32) -> i32 {
        if piles.len() == 1 {
            return piles[0] / h + if piles[0] % h == 0 { 0 } else { 1 };
        }
        piles.sort();
        let mut min = 1;
        let mut max = *piles.last().unwrap();
        while min < max {
            let mid = (min + max) / 2;
            let total = piles
                .iter()
                .map(|&v| v / mid + if v % mid == 0 { 0 } else { 1 })
                .sum::<i32>();
            if total <= h {
                max = mid;
            } else {
                min = mid + 1;
            }
        }
        min
    }
}
fn main() {
    println!(
        "{}",
        Solution::min_eating_speed(
            vec![
                332484035, 524908576, 855865114, 632922376, 222257295, 690155293, 112677673,
                679580077, 337406589, 290818316, 877337160, 901728858, 679284947, 688210097,
                692137887, 718203285, 629455728, 941802184
            ],
            823855818
        )
    );
}
